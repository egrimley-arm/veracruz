#!/bin/sh

set -e

export NATIVE_CC=gcc

export WASI_VERSION=12
export WASI_VERSION_FULL=${WASI_VERSION}.0
export WASI_SDK_PATH=/data/wasi-sdk/wasi-sdk-${WASI_VERSION_FULL}
export WASM_CC="${WASI_SDK_PATH}/bin/clang "\
"--sysroot=${WASI_SDK_PATH}/share/wasi-sysroot"
export WASM_RANLIB=/data/wasi-sdk/wasi-sdk-12.0/bin/ranlib

export CFLAGS='-DMBEDTLS_USER_CONFIG_FILE=\"mbedtls_user_config.h\"'\
" -I$(pwd)/include"

if ! [ -d mbedtls ] ; then
    rm -rf mbedtls mbedtls.tmp
    git clone https://github.com/ARMmbed/mbedtls.git mbedtls.tmp
    cd mbedtls.tmp
    git checkout mbedtls-3.0.0
    cd ..
    mv -T mbedtls.tmp mbedtls
fi

if ! [ -f native/libmbedcrypto.a ] ; then
    mkdir -p native
    rm -f native/libmbedcrypto.a
    cd mbedtls
    make clean
    CC=$NATIVE_CC make lib
    ln library/libmbedcrypto.a ../native/
    cd ..
fi

if ! [ -f wasm/libmbedcrypto.a ] ; then
    mkdir -p wasm
    rm -f wasm/libmbedcrypto.a
    cd mbedtls
    make clean
    CC=$WASM_CC make lib
    $WASM_RANLIB library/libmbedcrypto.a
    ln library/libmbedcrypto.a ../wasm/
    cd ..
fi

if ! [ -f native/prog ] ; then
    rm -f native/prog
    $NATIVE_CC -Wall -O2 psa-aes.c -Imbedtls/include -Lnative -lmbedcrypto \
        -o native/prog
fi

if ! [ -f wasm/prog ] ; then
    rm -f wasm/prog
    $WASM_CC -Wall -O2 psa-aes.c -Imbedtls/include -Lwasm -lmbedcrypto \
        -o wasm/prog
fi

echo Native:
native/prog

echo Wasm:
cd ../freestanding-execution-engine
cargo run -- --program ../crypto/wasm/prog -e true -o true --enable-clock true
