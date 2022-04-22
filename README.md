# Pure Rust for Android Proof of Concept
This project's aim is to have Rust code working on Android,with minimal effort by using the apk-rs plugin for Cargo, and also use Rust language directives to set the entry point for the Application.

This is intended to be a learning exercise for Rust and for the Android platform.

## Authors
Ben Cottrell

## Prerequisites
 - Android SDK

## Building
1. Install the apk-rs plugin first, if you haven't already, with:
`cargo install apk-rs`.
2. Make sure your device, whether physical or virtual, has been started, and that debugging is enabled for it. You may also need to allow your host machine to connect to your device when prompted to.
3. Run this, replacing the ndk version with the one you have installed:

`ANDROID_SDK_ROOT=<path to android sdk> ANDROID_NDK_ROOT=<path to android sdk>/ndk/<ndk version>/ cargo apk run`

## Debugging
In another terminal window, run this to display messages from the stream redirected to for Rust's stdout:

`PATH=<path to android sdk>/platform-tools/ adb logcat RustStdoutStderr:D *:S`
