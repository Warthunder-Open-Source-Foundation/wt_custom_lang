FROM archlinux:latest
ENV PATH=$PATH:/root/.cargo/bin
RUN pacman --noconfirm -Syu zsh git glib2 cmake rustup gtk3 base-devel mingw-w64-gcc
RUN pacman --noconfirm -Scc
RUN rustup toolchain install stable-x86_64-unknown-linux-gnu
RUN rustup target add x86_64-pc-windows-gnu
WORKDIR /wt_custom_lang
COPY ./start.sh .
RUN ["chmod", "+x", "./start.sh"]
CMD ["./start.sh"]