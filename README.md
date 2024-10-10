# Android Rust Example

## Setup

1. Install [`bazelisk`](https://bazel.build/install/bazelisk).
2. Install Android Studio, including the NDK. This repo is tested against NDK version 25.1.8937393.
3. Set `$ANDROID_HOME` in your environment to the Android SDK path (e.g. `/Users/user/Library/Android/sdk`), and set `$ANDROID_NDK_HOME` to the Android NDK path (e.g. `/Users/user/Library/Android/sdk/ndk/<version-number>`).
4. Install a device emulator for API level 34 or higher and run it.
5. Build and install the app to the emulator.
6. View standard output from Rust in Android Studio's Logcat by filtering for `example.rust.native` (some other stdout may be included).

Commands:

```shell
# Build the app
bazel build //android:app --config=android

# Install the app (also builds if needed)
bazel run //android:install --config=android

# Run the basic test, which checks that the app didn't crash on an assert.
bazel test //android:test_no_crash --config=android --test_env=ANDROID_HOME=$ANDROID_HOME
```

You can use [`ibazel`](https://github.com/bazelbuild/bazel-watcher) to automatically build/run/test on code changes.

It may be necessary to make the sh binaries `android/install.sh` and `android/test_no_crash.sh` runnable with `chmod +x`.
