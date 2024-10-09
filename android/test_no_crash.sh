#!/bin/bash

# Very basic test that the app can start up without crashing right away.
# An emulator must already be running.

set -euo pipefail
IFS=$'\n\t'

echo "Deleting previous app and data"
$ANDROID_HOME/platform-tools/adb uninstall example.rust

echo "Installing app"
$ANDROID_HOME/platform-tools/adb install android/app.apk

echo "Starting app"
$ANDROID_HOME/platform-tools/adb shell monkey -p example.rust 1

# wait for app to start
sleep 5

# check that the app is still running and didn't crash
$ANDROID_HOME/platform-tools/adb shell ps | grep example.rust
