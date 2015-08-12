#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <lz4.h>
#include <lz4hc.h>
#include <lzma.h>
#include <assert.h>
#include <skr.h>

static int
lz4(const uint8_t *in, size_t nin, uint8_t **out, size_t nout) {
  const size_t limit = LZ4_compressBound(nin);
  if (nout < limit) {
    nout = limit;
    *out = realloc(*out, nout);
  }
  nout = LZ4_compressHC((void*) in, (void*) *out, nin);
  if (nout)
    return nout;
  return -1;
}

static int
unlz4(const uint8_t *in, size_t nin, uint8_t **out, size_t nout) {
  const size_t limit = 256 * 1024;
  if (nout < limit) {
    nout = limit;
    *out = realloc(*out, nout);
  }
  int real_nout = LZ4_decompress_safe((void*) in, (void*) *out, nin, nout);
  if (real_nout >= 0)
    return real_nout;
  return -1;
}

/*
 * Based on 01_compress_easy.c and 02_decompress.c by Lasse Collin.
 */

static int
err(lzma_ret ret) {
  const char *msg;
  switch (ret) {
    case LZMA_MEM_ERROR:
      msg = "Memory allocation failed";
      break;

    case LZMA_OPTIONS_ERROR:
      msg = "Unsupported flags";
      break;

    case LZMA_UNSUPPORTED_CHECK:
      msg = "Specified integrity check is not supported";
      break;

    case LZMA_FORMAT_ERROR:
      msg = "The input is not in the .lzma format";
      break;

    case LZMA_DATA_ERROR:
      msg = "Compressed file is corrupt";
      break;

    case LZMA_BUF_ERROR:
      msg = "Compressed file is truncated or otherwise corrupt";
      break;

    default:
      msg = "Unknown error, possibly a bug";
      break;
  }
  fprintf(stderr, "Error initializing LZMA: %s (error code %u)\n", msg, ret);
  return -1;
}

static int
init_encoder(lzma_stream *strm) {
  lzma_options_lzma opts;
  lzma_lzma_preset(&opts, LZMA_PRESET_DEFAULT | LZMA_PRESET_EXTREME);
  lzma_ret ret = lzma_alone_encoder(strm, &opts);

  if (ret == LZMA_OK)
    return 0;

  return err(ret);
}

static int
init_decoder(lzma_stream *strm) {
  lzma_ret ret = lzma_alone_decoder(strm, UINT64_MAX);

  if (ret == LZMA_OK)
    return 0;

  return err(ret);
}

static int
pipe(lzma_stream *strm, const uint8_t *in, size_t nin, uint8_t **out, size_t nout) {
  uint8_t outbuf[1024];
  size_t nread = 0;
  strm->next_in = in;
  strm->avail_in = nin;
  strm->next_out = outbuf;
  strm->avail_out = sizeof(outbuf);
  lzma_action action = LZMA_RUN;
  for (;;) {
    if (strm->avail_in == 0)
      action = LZMA_FINISH;

    lzma_ret ret = lzma_code(strm, action);

    if (strm->avail_out == 0 || ret == LZMA_STREAM_END) {
      size_t write_size = sizeof(outbuf) - strm->avail_out;
      if (write_size + nread > nout) {
        nout += write_size;
        *out = realloc(*out, nout);
      }
      memcpy(*out + nread, outbuf, write_size);
      nread += write_size;

      strm->next_out = outbuf;
      strm->avail_out = sizeof(outbuf);
    }

    if (ret != LZMA_OK) {
      if (ret == LZMA_STREAM_END)
        return nout;
      else
        return err(ret);
    }
  }
}

static int
lzma(const uint8_t *in, size_t nin, uint8_t **out, size_t nout) {
  lzma_stream strm = LZMA_STREAM_INIT;
  if (init_encoder(&strm) < 0)
    return -1;
  int ret = pipe(&strm, in, nin, out, nout);
  lzma_end(&strm);
  return ret;
}

static int
unlzma(const uint8_t *in, size_t nin, uint8_t **out, size_t nout) {
  lzma_stream strm = LZMA_STREAM_INIT;
  if (init_decoder(&strm) < 0)
    return -1;
  int ret = pipe(&strm, in, nin, out, nout);
  lzma_end(&strm);
  return ret;
}

typedef int ((fun_t)(const uint8_t *, size_t, uint8_t **, size_t));

typedef const struct {
  fun_t* comp;
  fun_t* decomp;
} funs_t;

static funs_t function_pairs[] = {
  { &lzma, &unlzma },
  { &lz4, &unlz4 },
};

static funs_t*
functions_for_options(skr_opts_t* opts) {
  switch (opts->opts & 1) {
    case SKR_LZMA:
      return function_pairs;
    case SKR_LZ4:
      return function_pairs + 1;
    default:
      return 0;
  }
}

/*
 * Implementation of the API defined in skr.h
 */

int
skr_model(const uint8_t* input, size_t input_len,
          uint8_t** output, size_t output_len,
          skr_opts_t* opts) {
  funs_t *fns = functions_for_options(opts);
  return fns->comp(input, input_len, output, output_len);
}

int
skr_compress(const uint8_t* model, size_t model_len,
             const uint8_t* input, size_t input_len,
             uint8_t** output, size_t output_len,
             skr_opts_t* opts) {
  uint8_t *full_model = 0, *compr = 0;
  funs_t *fns = functions_for_options(opts);
  size_t full_model_len = fns->decomp(model, model_len, &full_model, 0);
  full_model = realloc(full_model, full_model_len + input_len);
  memcpy(full_model + full_model_len, input, input_len);
  size_t compr_len = fns->comp(full_model, full_model_len + input_len, &compr, 0);
  free(full_model);
  size_t idx = 0;
  while (compr[idx] == model[idx])
    ++idx;
  size_t needed_out_len = compr_len - idx + 1;
  if (output_len < needed_out_len)
    *output = realloc(*output, needed_out_len);
  size_t diff = model_len - idx;
  assert(diff <= 0xff);
  **output = (uint8_t) diff;
  memcpy(*output + 1, compr + idx, needed_out_len - 1);
  free(compr);
  return needed_out_len;
}

int
skr_decompress(const uint8_t* model, size_t model_len,
               const uint8_t* input, size_t input_len,
               uint8_t** output, size_t output_len,
               skr_opts_t* opts) {
  uint8_t *buffer = 0, *decomp = 0;
  funs_t *fns = functions_for_options(opts);
  size_t full_model_len = fns->decomp(model, model_len, &buffer, 0);
  size_t cutoff = model_len - input[0];
  size_t patched_len = cutoff + input_len - 1;
  if (full_model_len < patched_len)
    buffer = realloc(buffer, patched_len);
  memcpy(buffer, model, cutoff);
  memcpy(buffer + cutoff, input + 1, input_len - 1);
  size_t decompr_len = fns->decomp(buffer, patched_len, &decomp, 0);
  free(buffer);
  size_t needed_out_len = decompr_len - full_model_len;
  if (output_len < needed_out_len)
    *output = realloc(*output, needed_out_len);
  memcpy(*output, decomp + full_model_len, needed_out_len);
  free(decomp);
  return needed_out_len;
}
