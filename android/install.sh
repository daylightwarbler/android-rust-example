#!/bin/bash

# Install and start the app

set -euo pipefail
IFS=$'\n\t'

echo "Installing app"
$ANDROID_HOME/platform-tools/adb install android/app.apk

echo "Starting app"
$ANDROID_HOME/platform-tools/adb shell monkey -p example.rust 1
