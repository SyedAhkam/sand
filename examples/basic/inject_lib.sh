#!/usr/bin/env bash

TARGETS=-t\ armeabi-v7a\ -t\ arm64-v8a
OUTPUT=android/app/src/main/jniLibs

cargo ndk $TARGETS -o "$OUTPUT" build
