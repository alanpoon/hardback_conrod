#!/bin/bash

set -eu

# Android Platform Version
ANDROID_PLATFORM="android-21"
# NDK home
NDK_HOME="/usr/local/lib/android/Sdk/android-ndk-r12b"
# Output Directory Path
OUTPUT_DIR="/usr/local/ssl/$ANDROID_PLATFORM"

# Android NDK standalone-toolchain Path
NDK_TOOLCHAIN_PATH="${OUTPUT_DIR}/toolchain"

# Android NDK standalone-toolchain(ARM)
ARM_TOOLCHAIN="arm-linux-androideabi-4.9"

# Android NDK standalone-toolchain(x86)
X86_TOOLCHAIN="x86-4.9"

# Android NDK standalone-toolchain Path(ARM)
ARM_TOOLCHAIN_PATH="${NDK_TOOLCHAIN_PATH}/arm"

# Android NDK standalone-toolchain Path(x86)
X86_TOOLCHAIN_PATH="${NDK_TOOLCHAIN_PATH}/x86"

# Android NDK standalone-toolchain Prefix(ARM)
ARM_TOOLCHAIN_PREFIX="${ARM_TOOLCHAIN_PATH}/bin/arm-linux-androideabi-"

# Android NDK standalone-toolchain Prefix(x86)
X86_TOOLCHAIN_PREFIX="${X86_TOOLCHAIN_PATH}/bin/i686-linux-android-"


################################################################################
#
################################################################################

# make Android NDK standalone-toolchain(ARM)
rm -rf "${ARM_TOOLCHAIN_PATH}"
"${NDK_HOME}/build/tools/make-standalone-toolchain.sh" --arch=arm --toolchain="${ARM_TOOLCHAIN}" --install-dir="${ARM_TOOLCHAIN_PATH}" --platform="${ANDROID_PLATFORM}"

# make Android NDK standalone-toolchain(x86)
rm -rf "${X86_TOOLCHAIN_PATH}"
"${NDK_HOME}/build/tools/make-standalone-toolchain.sh" --arch=x86 --toolchain="${X86_TOOLCHAIN}" --install-dir="${X86_TOOLCHAIN_PATH}" --platform="${ANDROID_PLATFORM}"

################################################################################
#
################################################################################


# build for armeabi
CONFIG="android"
PREFIX="${OUTPUT_DIR}/armeabi"
CROSS_COMPILE_PREFIX="${ARM_TOOLCHAIN_PREFIX}"
export ANDROID_DEV="${NDK_HOME}/platforms/${ANDROID_PLATFORM}/arch-arm/usr"
./Configure --prefix="${PREFIX}" --cross-compile-prefix="${CROSS_COMPILE_PREFIX}" "${CONFIG}" threads no-shared
make -s && make install

# build for armeabi-v7a
CONFIG="android-armv7"
PREFIX="${OUTPUT_DIR}/armeabi-v7a"
CROSS_COMPILE_PREFIX="${ARM_TOOLCHAIN_PREFIX}"
export ANDROID_DEV="${NDK_HOME}/platforms/${ANDROID_PLATFORM}/arch-arm/usr"
make clean
./Configure --prefix="${PREFIX}" --cross-compile-prefix="${CROSS_COMPILE_PREFIX}" "${CONFIG}" threads no-shared
make -s && make install

# build for x86
CONFIG="android-x86"
PREFIX="${OUTPUT_DIR}/x86"
CROSS_COMPILE_PREFIX="${X86_TOOLCHAIN_PREFIX}"
export ANDROID_DEV="${NDK_HOME}/platforms/${ANDROID_PLATFORM}/arch-x86/usr"
make clean
./Configure --prefix="${PREFIX}" --cross-compile-prefix="${CROSS_COMPILE_PREFIX}" "${CONFIG}" threads no-shared
make -s && make install

cp -f "${SCRIPT_DIR}/Android.mk" "${OUTPUT_DIR}/Android.mk"
rm -rf "${NDK_TOOLCHAIN_PATH}"

export OPENSSL_DIR="/usr/local/ssl/android-21/armeabi-v7a"
export OPENSSL_LIB_DIR="/usr/local/ssl/android-21/armeabi-v7a/lib"