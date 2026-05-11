ARG BUILD_PLATFORM=linux/amd64
FROM --platform=${BUILD_PLATFORM} rust:alpine

WORKDIR /workspace

RUN rustup toolchain install nightly && rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-musl

RUN apk update && apk add --no-cache \
    make \
	nasm \
    grub \
	grub-bios \
    mtools \
    xorriso \
    libisoburn \
    qemu-ui-curses \
    ncurses \
    qemu-system-i386

COPY Cargo.toml Cargo.lock rust-toolchain.toml ./

COPY tools/ ./tools/
COPY .cargo/ ./.cargo/
COPY tests/ ./tests/

ENV BUILD_DIR="build"
RUN mkdir $BUILD_DIR

COPY src/ ./src/
RUN cargo build -Zjson-target-spec

COPY Makefile ./

CMD ["make"]