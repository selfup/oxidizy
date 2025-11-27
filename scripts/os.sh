#!/usr/bin/env bash

if command -v sudo &> /dev/null
then
    RUN_CMD="sudo"
    echo "--- sudo found, using sudo for privileged commands ---"
else
    RUN_CMD=""
    echo "--- sudo not found, running commands without sudo ---"
fi

echo "--- Attempting to update system package list... ---"

$RUN_CMD apt-get update

echo "--- Attempting to add deps for bevy to run in pipeline... ---"

$RUN_CMD apt-get install g++ \
    pkg-config libx11-dev \
    libasound2-dev libudev-dev \
    libwayland-dev -y

echo "--- OS dep CI script finished ---"
