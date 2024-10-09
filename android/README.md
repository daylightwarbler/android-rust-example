# Android App

Based on the [Bazel Kotlin example](https://github.com/bazelbuild/examples/tree/d18ce42623f136616e9512d713a47c33f9ad6a58/android/jetpack-compose)

## Start Emulator

```
$ANDROID_HOME/emulator/emulator -list-avds
$ANDROID_HOME/emulator/emulator -avd Medium_Phone_API_34
```

## Building

Build and run the app in the emulator:

```
bazel build :app --config=android
bazel run :install --config=android
```

It may be necessary to make the sh binary runnable with `chmod +x install.sh`.
