FROM archlinux:latest
ENV PATH=$PATH:/root/.cargo/bin

RUN pacman --noconfirm -Syu zsh git glib2 cmake rustup gtk3 base-devel mingw-w64-gcc github-cli # Necessary development and deployment tools
RUN pacman --noconfirm -Scc # Cleaning cache for a small image

RUN rustup toolchain install stable-x86_64-unknown-linux-gnu # For building the linux image, and having all toolchain utilities for building
RUN rustup target add x86_64-pc-windows-gnu # Cross-compiling to windows requires this

WORKDIR /flareflo-wt

# The token is for publishing the release to git
COPY ./token.txt .

# Main build script
COPY ./start.sh .
CMD ["./start.sh"]