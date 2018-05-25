function getAndroidSDK {
  export PATH="$ANDROID_HOME/platform-tools:$ANDROID_HOME/tools:$PATH"

  DEPS="$ANDROID_HOME/installed-dependencies"

  if [ ! -e $DEPS ]; then
    echo y | android update sdk -u -a -t android-19 &&
    echo y | android update sdk -u -a -t platform-tools &&
    echo y | android update sdk -u -a -t build-tools-21.1.2 &&
    echo y | android update sdk -u -a -t extra-android-m2repository &&
    echo y | android update sdk -u -a -t extra-android-support &&
    echo y | android update sdk -u -a -t extra-google-m2repository &&
    echo no | android create avd -n testAVD -f -t android-19 --abi default/armeabi-v7a &&
    touch $DEPS
  fi
}