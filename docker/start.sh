#!/bin/zsh
rustc --print target-list

echo "Cloning..."
git clone https://github.com/Warthunder-Open-Source-Foundation/wt_custom_lang.git

echo "Going to git folder"
cd wt_custom_lang

echo "Setting backtrace to 1"
export RUST_BACKTRACE=1

echo "Building linux"
cargo build

echo "Building"
cargo build --target 'x86_64-pc-windows-gnu'