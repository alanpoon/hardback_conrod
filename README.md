    rustup target add arm-linux-androideabi
    cargo install cargo-apk
    git clone https://github.com/jbg/conrod-android-skeleton && cd conrod-android-skeleton
    cargo apk build
    adb install -r target/android-artifacts/app/build/outputs/apk/app-debug.apk
