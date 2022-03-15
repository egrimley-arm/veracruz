
#include <stdio.h>
#include <stdlib.h>

#include <psa/crypto.h>

void fail(const char *s)
{
    fprintf(stderr, "Error: %s\n", s);
    exit(1);
}

void usage(const char *prog)
{
    printf("Usage: %s BLOCK_SIZE KEYFILE\n", prog);
    exit(0);
}

int main(int argc, char *argv[])
{
    const char *prog = argv[0];
    if (argc != 3)
        usage(prog);

    size_t block_size;
    {
        char *endptr;
        block_size = strtol(argv[1], &endptr, 0);
        if (!block_size || *endptr)
            usage(prog);
    }
    const char *keyfile = argv[2];

    unsigned char key[16] = { 0 };
    {
        FILE *fp = fopen(keyfile, "r");
        if (!fp)
            fail("fopen keyfile");
        fread(key, 1, sizeof(key), fp);
        if (ferror(fp))
            fail("fread keyfile");
        fclose(fp);
    }

    unsigned char *data = calloc(block_size, 1);
    unsigned char *output = calloc(block_size, 1);
    if (!data || !output)
        fail("calloc");

    if (psa_crypto_init() != PSA_SUCCESS)
        fail("psa_crypto_init");

    psa_key_attributes_t attr = PSA_KEY_ATTRIBUTES_INIT;
    psa_set_key_usage_flags(&attr,
                            PSA_KEY_USAGE_DECRYPT | PSA_KEY_USAGE_ENCRYPT);
    psa_set_key_algorithm(&attr, PSA_ALG_CTR);
    psa_set_key_type(&attr, PSA_KEY_TYPE_AES);

    mbedtls_svc_key_id_t key_id;
    if (psa_import_key(&attr, key, sizeof(key), &key_id) != PSA_SUCCESS)
        fail("psa_import_key");
    psa_cipher_operation_t operation = { 0 };
    if (psa_cipher_encrypt_setup(&operation, key_id, PSA_ALG_CTR) !=
        PSA_SUCCESS)
        fail("psa_cipher_encrypt_setup");

    unsigned char iv[16] = { 0 };
    if (psa_cipher_set_iv(&operation, iv, sizeof(iv)) !=
        PSA_SUCCESS)
        fail("psa_cipher_set_iv");

    for (;;) {
        size_t sz = fread(data, 1, block_size, stdin);
        if (ferror(stdin))
            fail("fread stdin");
        if (!sz)
            break;

        size_t output_length = 0;
        if (psa_cipher_update(&operation, data, sz, output, block_size,
                              &output_length) != PSA_SUCCESS)
            fail("psa_cipher_update");
        fwrite(output, 1, output_length, stdout);
        if (ferror(stdout))
            fail("fwrite stdout");
    }
    size_t output_length = 0;
    if (psa_cipher_finish(&operation, output, block_size, &output_length) !=
        PSA_SUCCESS)
        fail("psa_cipher_finish");
    fwrite(output, 1, output_length, stdout);
    if (ferror(stdout))
        fail("fwrite stdout");

    return 0;
}
