/*
 * Based on 01_compress_easy.c and 02_decompress.c by Lasse Collin.
 */

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <lzma.h>

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
pipe(lzma_stream *strm, uint8_t *in, size_t nin, uint8_t **out, size_t nout) {
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

      const char *msg;
      switch (ret) {
        case LZMA_MEM_ERROR:
          msg = "Memory allocation failed";
          break;

        case LZMA_FORMAT_ERROR:
          msg = "The input is not in the .xz format";
          break;

        case LZMA_OPTIONS_ERROR:
          msg = "Unsupported compression options";
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

      fprintf(stderr, "Decoder error: %s (error code %u)\n", msg, ret);
      return -1;
    }
  }
}

int
lzma (uint8_t *in, size_t nin, uint8_t **out, size_t nout) {
  lzma_stream strm = LZMA_STREAM_INIT;
  if (init_encoder(&strm) < 0)
    return -1;
  int ret = pipe(&strm, in, nin, out, nout);
  lzma_end(&strm);
  return ret;
}

int
unlzma (uint8_t *in, size_t nin, uint8_t **out, size_t nout) {
  lzma_stream strm = LZMA_STREAM_INIT;
  if (init_decoder(&strm) < 0)
    return -1;
  int ret = pipe(&strm, in, nin, out, nout);
  lzma_end(&strm);
  return ret;
}
