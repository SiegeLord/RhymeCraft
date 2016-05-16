#!/bin/bash

export ALLEGRO_INCLUDE_DIR=allegro/include
export ALLEGRO_LINK_PATH=allegro/lib
cargo build --release
