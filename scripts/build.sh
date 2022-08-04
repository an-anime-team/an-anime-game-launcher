#!/bin/bash

curr_dir=$(dirname "$0")

cd $(dirname $curr_dir)

cargo build --release

cd $curr_dir

rm -rf builds
mkdir builds

cp ../target/release/anime-game-launcher builds/anime-game-launcher

./appimage/build_appimage.sh
