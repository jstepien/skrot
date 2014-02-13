#ifndef SKR_H
#define SKR_H

#include <stdint.h>

int skr_corpus(uint8_t* input, size_t input_len,
               uint8_t** output, size_t output_len);

int skr_compress(uint8_t* corpus, size_t corpus_len,
                 uint8_t* input, size_t input_len,
                 uint8_t** output, size_t output_len);

int skr_decompress(uint8_t* corpus, size_t corpus_len,
                   uint8_t* input, size_t input_len,
                   uint8_t** output, size_t output_len);

#endif /* SKR_H */
