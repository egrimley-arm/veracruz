#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <psa/crypto.h>

#define assert(x) do { if (!(x)) { \
        assert_fail(__FILE__, __LINE__, __func__, #x); } } while (0)

void assert_fail(const char *file, unsigned long line,
                 const char *func, const char *cond)
{
    fprintf(stderr, "%s:%ld: %s: Assertion '%s' failed.\n",
            file, line, func, cond);
    exit(1);
}

int mbedtls_hardware_poll(void *data,
                          unsigned char *output, size_t len, size_t *olen)
{
    memset(output, 0, len);
    *olen = len;
    return 0;
}

int main()
{
    assert(!psa_crypto_init());

    psa_key_attributes_t attributes = PSA_KEY_ATTRIBUTES_INIT;
    psa_set_key_type(&attributes, PSA_KEY_TYPE_AES);
    psa_set_key_algorithm(&attributes, PSA_ALG_CBC_NO_PADDING);
    psa_set_key_usage_flags(&attributes,
                            PSA_KEY_USAGE_ENCRYPT | PSA_KEY_USAGE_DECRYPT);
    uint8_t data[32] = { 0 };
    mbedtls_svc_key_id_t key = 0;
    assert(!psa_import_key(&attributes, data, sizeof(data), &key));

    psa_algorithm_t alg = PSA_ALG_CBC_NO_PADDING;
    uint8_t plain[16] = { 1, 2, 3 };
    uint8_t cypher[32] = { 0 };
    size_t cypher_len = 0;
    assert(!psa_cipher_encrypt(key, alg, plain, sizeof(plain),
                               cypher, sizeof(cypher), &cypher_len));

    uint8_t plain2[16] = { 0 };
    size_t plain_len = 0;
    assert(!psa_cipher_decrypt(key, alg, cypher, cypher_len,
                               plain2, sizeof(plain2), &plain_len));

    assert(plain_len == sizeof(plain) &&
           !memcmp(plain, plain2, sizeof(plain)));

    for (size_t i = 0; i < cypher_len; i++)
        printf(" %02x", cypher[i]);
    printf("\n");

    return 0;
}
