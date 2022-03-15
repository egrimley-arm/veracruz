#!/bin/sh

# Test AES counter-mode encryption/decryption using three different APIs:
# * Mbed TLS
# * PSA Crypto
# * rust-psa-crypto

set -e

printf '%s' SECRET > key

gcc -Wall -g \
  test-mbedtls.c -o test-mbedtls \
  -I../third-party/rust-psa-crypto/psa-crypto-sys/vendor/include \
  -L../third-party/rust-psa-crypto/psa-crypto-sys/vendor/library -lmbedcrypto
./test-mbedtls 7 key < test.sh > data1
./test-mbedtls 11 key < data1 > data1b
diff test.sh data1b

gcc -Wall -g \
  test-psacrypto.c -o test-psacrypto \
  -I../third-party/rust-psa-crypto/psa-crypto-sys/vendor/include \
  -L../third-party/rust-psa-crypto/psa-crypto-sys/vendor/library -lmbedcrypto
./test-psacrypto 7 key < test.sh > data2
./test-psacrypto 11 key < data2 > data2b
diff test.sh data2b

diff data1 data2

cargo build
target/debug/test-rust-psa-crypto < test.sh > data3
target/debug/test-rust-psa-crypto < data3 > data3b
diff test.sh data3b

diff data1 data3

echo PASS
