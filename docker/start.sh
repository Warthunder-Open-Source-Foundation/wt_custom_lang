#!/bin/zsh
echo "Loging into GH"
gh auth login --with-token < token.txt

echo "Cloning..."
git clone https://github.com/Warthunder-Open-Source-Foundation/wt_custom_lang.git

echo "Entering git folder"
cd wt_custom_lang

echo "Setting backtrace to 1"
export RUST_BACKTRACE=1

echo "Pre-fetching cargo crates"
cargo fetch

echo "Building linux target"
cargo build -j 10 --release --target x86_64-unknown-linux-gnu # The -j flag can be removed to use 100% of the available threads

echo "Building"
cargo build -j 10 --release --target x86_64-pc-windows-gnu

# The steps below are only required for publishing to github

echo "Preparing files for release"
mv ./target/x86_64-pc-windows-gnu/release/wt_custom_lang.exe ./wt_custom_lang_windows_x86_64.exe
mv ./target/x86_64-unknown-linux-gnu/release/wt_custom_lang ./wt_custom_lang_linux_x86_64

echo "Creating release"
gh release create v1.0.0 --generate-notes ./wt_custom_lang_windows_x86_64.exe ./wt_custom_lang_linux_x86_64 ./LICENSE ./NOTICE