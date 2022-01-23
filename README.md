#<a href="https://github.com/Warthunder-Open-Source-Foundation/wt_custom_lang/blob/master/guide/simple_installation.md" title="simple installation ">Simple installation guide </a>

# For using the tool, please refer to <a href="https://github.com/Warthunder-Open-Source-Foundation/wt_custom_lang/blob/master/guide/how_to_use.md" title="guide">here</a>

***

## Building the project from source using crates.io
1. <a href="https://www.rust-lang.org/tools/install">Install Rustup</a>
2. run ``cargo install wt_custom_lang``

## Building the project from git
1. <a href="https://www.rust-lang.org/tools/install">Install Rustup</a>
2. Clone the repository (master will always compile, nightly might not)
3. Run ``cargo build --release`` to build, or ``cargo run --release`` to run directly

## Building the project using docker
1. Make sure to have a running Docker installation
2. <a href="https://github.com/Warthunder-Open-Source-Foundation/wt_custom_lang/tree/master/docker">Modify the dockerfile and start script to suit the build target</a>
3. Run the image