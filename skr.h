#ifndef SKR_H
#define SKR_H

#include <stdint.h>

typedef struct {
  uint32_t opts;
} skr_opts_t;

enum {
  SKR_LZMA = 0,
  SKR_LZ4  = 1,
};

int skr_model(const uint8_t* input, size_t input_len,
              uint8_t** output, size_t output_len,
              skr_opts_t* opts);

int skr_compress(const uint8_t* model, size_t model_len,
                 const uint8_t* input, size_t input_len,
                 uint8_t** output, size_t output_len,
                 skr_opts_t* opts);

int skr_decompress(const uint8_t* model, size_t model_len,
                   const uint8_t* input, size_t input_len,
                   uint8_t** output, size_t output_len,
                   skr_opts_t* opts);

#endif /* SKR_H */
