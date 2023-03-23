#!/bin/bash

cargo build --release

cp -rf ./target/release/font_helper ./res/fonthelper

cd ./res

tar cJf ../fonthelper.tar.xz ./{figma-fonthelper*,fonthelper*,updater.sh,libfreetype*}
