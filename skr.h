#ifndef SKR_H
#define SKR_H

#include <stdint.h>

int skr_model(const uint8_t* input, size_t input_len,
              uint8_t** output, size_t output_len);

int skr_compress(const uint8_t* model, size_t model_len,
                 const uint8_t* input, size_t input_len,
                 uint8_t** output, size_t output_len);

int skr_decompress(const uint8_t* model, size_t model_len,
                   const uint8_t* input, size_t input_len,
                   uint8_t** output, size_t output_len);

#endif /* SKR_H */
