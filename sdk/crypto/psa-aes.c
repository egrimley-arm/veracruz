#include <errno.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <psa/crypto.h>

// The standard implementation of assert raises a signal on failure,
// which causes the Wasm engine to terminate in a confusing way with
// no indication of which assertion failed, so we use this instead:

#define assert(x) do { if (!(x)) { \
        assert_fail(__FILE__, __LINE__, __func__, #x); } } while (0)

void assert_fail(const char *file, unsigned long long line,
                 const char *func, const char *cond)
{
    fprintf(stderr, "%s:%llu: %s: Assertion '%s' failed.\n",
            file, line, func, cond);
    exit(1);
}

// This bogus hardware entropy collector is provided for use by
// Mbed TLS, which is configured with MBEDTLS_ENTROPY_HARDWARE_ALT:

int mbedtls_hardware_poll(void *data,
                          unsigned char *output, size_t len, size_t *olen)
{
    memset(output, 0, len);
    *olen = len;
    return 0;
}

void help(void)
{
    printf("Usage: prog [BUFLEN [REPEAT]]\n");
    exit(1);
}

int main(int argc, char *argv[])
{
    size_t plain_len = 16;
    size_t repeats = 1;
    if (argc > 1) {
        errno = 0;
        char *endp;
        unsigned long long x = strtoull(argv[1], &endp, 0);
        plain_len = x;
        if (errno || !*argv[1] || *endp || plain_len != x)
            help();
    }
    if (argc > 2) {
        errno = 0;
        char *endp;
        unsigned long long x = strtoull(argv[2], &endp, 0);
        repeats = x;
        if (errno || !*argv[2] || *endp || repeats != x)
            help();
    }
    if (argc > 3)
        help();

    assert(!psa_crypto_init());

    // Import key.
    psa_key_attributes_t attributes = PSA_KEY_ATTRIBUTES_INIT;
    psa_set_key_type(&attributes, PSA_KEY_TYPE_AES);
    psa_set_key_algorithm(&attributes, PSA_ALG_CBC_NO_PADDING);
    psa_set_key_usage_flags(&attributes,
                            PSA_KEY_USAGE_ENCRYPT | PSA_KEY_USAGE_DECRYPT);
    uint8_t data[32] = { 0 };
    mbedtls_svc_key_id_t key = 0;
    assert(!psa_import_key(&attributes, data, sizeof(data), &key));

    // Set up plaintext.
    uint8_t *plain = malloc(plain_len);
    assert(plain);
    for (size_t i = 0; i < plain_len; i++)
        plain[i] = i % 251;

    // Encrypt.
    size_t cypher_size = plain_len + 16;
    assert(cypher_size > plain_len);
    uint8_t *cypher = malloc(cypher_size);
    assert(cypher);
    psa_algorithm_t alg = PSA_ALG_CBC_NO_PADDING;
    size_t cypher_len = 0;
    assert(!psa_cipher_encrypt(key, alg, plain, plain_len,
                               cypher, cypher_size, &cypher_len));

    // Decrypt repeatedly.
    struct timespec ts1, ts2;
    uint8_t *plain2 = malloc(plain_len);
    assert(plain2);
    memset(plain2, 0, plain_len);
    assert(!clock_gettime(CLOCK_MONOTONIC, &ts1));
    for (size_t i = 0; i < repeats; i++) {
        size_t plain2_len = 0;
        assert(!psa_cipher_decrypt(key, alg, cypher, cypher_len,
                                   plain2, plain_len, &plain2_len));
        assert(plain2_len == plain_len);
    }
    assert(!clock_gettime(CLOCK_MONOTONIC, &ts2));

    // Check decryption worked.
    assert(!memcmp(plain, plain2, plain_len));

    printf("Message of size %zd decripted %zd times.\n"
           "Total time: %llu ns\n",
           plain_len, repeats,
           (ts2.tv_sec - ts1.tv_sec) * 1000000000ULL +
           (ts2.tv_nsec - ts1.tv_nsec));

    return 0;
}
