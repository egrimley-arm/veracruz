
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <mbedtls/aes.h>

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

    mbedtls_aes_context ctx;
    mbedtls_aes_init(&ctx);
    if (mbedtls_aes_setkey_enc(&ctx, key, 128))
        fail("mbedtls_aes_setkey_enc");
    unsigned char nonce_counter[16] = { 0 };
    unsigned char stream_block[16] = { 0 };
    size_t nc_off = 0;

    for (;;) {
        size_t sz = fread(data, 1, block_size, stdin);
        if (ferror(stdin))
            fail("fread stdin");
        if (!sz)
            break;

        if (mbedtls_aes_crypt_ctr(&ctx, sz, &nc_off,
                                  nonce_counter, stream_block,
                                  data, output))
            fail("mbedtls_aes_crypt_ctr");
        fwrite(output, 1, sz, stdout);
        if (ferror(stdout))
            fail("fwrite stdout");
    }
    mbedtls_aes_free(&ctx);
    return 0;
}
