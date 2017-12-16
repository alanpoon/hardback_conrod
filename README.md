    rustup target add arm-linux-androideabi
    cargo install cargo-apk
    git clone https://github.com/jbg/conrod-android-skeleton && cd conrod-android-skeleton
    cargo apk build
    adb install -r target/android-artifacts/app/build/outputs/apk/app-debug.apk

android
cargo apk run
-L ~/software/openssl-1.0.2l/libcrypto.a -l ssl -lcryto
adb install -r target/android-artifacts/app/build/outputs/apk/outfile.apk

need to add this line to android in cargo.toml
conrod_chat = {path ="../conrod_chat", features=["keypad"]}
