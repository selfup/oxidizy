#!/usr/bin/env bash

sudo apt-get update \
    && sudo apt-get install g++ \
        pkg-config libx11-dev \
        libasound2-dev libudev-dev \
        libwayland-dev -y
