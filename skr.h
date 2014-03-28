#ifndef SKR_H
#define SKR_H

#include <stdint.h>
#include <stdlib.h>

/*
 * (De)compression options. So far its only purpose is deciding whether we want
 * to (de)compress with LZMA or LZ4. Only valid values are:
 *
 *   - 0 for LZMA, and
 *   - 1 for LZ4.
 */
typedef struct {
  uint32_t opts;
} skr_opts_t;

enum {
  SKR_LZMA = 0,
  SKR_LZ4  = 1,
};

/*
 * skr_model
 *
 * Builds a Skrot model. Arguments are:
 *
 *   - input - an input byte array,
 *   - input_len - its length,
 *   - output - a pointer to a malloc'ed output byte array,
 *   - output_len - size of the output byte array, and
 *   - opts - (de)compression options, see skr_opts_t.
 *
 * output will be realloc'ed if its size is insufficient. output can point to
 * a null pointer; in such case output_len should be 0.
 *
 * Returns size of the output array in case of success. Otherwise a negative
 * value is returned.
 */
int skr_model(const uint8_t* input, size_t input_len,
              uint8_t** output, size_t output_len,
              skr_opts_t* opts);

/*
 * skr_compress
 *
 * Compresses given byte array using the given model. Arguments are:
 *
 *   - model - a Skrot model build with skr_model,
 *   - model_len - length of the model array,
 *   - input - an input byte array,
 *   - input_len - its length,
 *   - output - a pointer to a malloc'ed output byte array,
 *   - output_len - size of the output byte array, and
 *   - opts - (de)compression options, see skr_opts_t.
 *
 * See skr_model for comments regarding the output argument.
 *
 * Return values same as in case of skr_model.
 */
int skr_compress(const uint8_t* model, size_t model_len,
                 const uint8_t* input, size_t input_len,
                 uint8_t** output, size_t output_len,
                 skr_opts_t* opts);

/*
 * skr_decompress
 *
 * Decompresses given byte array using the given model. Arguments are:
 *
 *   - model - a Skrot model build with skr_model,
 *   - model_len - length of the model array,
 *   - input - an input byte array,
 *   - input_len - its length,
 *   - output - a pointer to a malloc'ed output byte array,
 *   - output_len - size of the output byte array, and
 *   - opts - (de)compression options, see skr_opts_t.
 *
 * See skr_model for comments regarding the output argument.
 *
 * Return values same as in case of skr_model.
 */
int skr_decompress(const uint8_t* model, size_t model_len,
                   const uint8_t* input, size_t input_len,
                   uint8_t** output, size_t output_len,
                   skr_opts_t* opts);

#endif /* SKR_H */
