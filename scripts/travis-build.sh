#!/usr/bin/env bash
#
# Build script for Travis CI

if [ "$TRAVIS_OS_NAME" = "osx" ]; then
  echo "Special build rules for OS X here"
fi

if [ "$TRAVIS_OS_NAME" = "linux" ]; then
  echo "Special build rules for Linux here"
  sudo apt update -y
  sudo apt install -y libsdl2-dev libsdl2-ttf-dev libsdl2-image-dev
fi

echo "Building Pushrod"

cargo build --tests

