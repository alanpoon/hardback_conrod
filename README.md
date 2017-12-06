    rustup target add arm-linux-androideabi
    cargo install cargo-apk
    git clone https://github.com/jbg/conrod-android-skeleton && cd conrod-android-skeleton
    cargo apk build
    adb install -r target/android-artifacts/app/build/outputs/apk/app-debug.apk

hand [144, 159, 157, 140, 146]
draft [157, 159, 148, 144, 142, 146, 140, 151, 158, 153]