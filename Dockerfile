# 1.56.1 as of now (2021-11-14)
FROM liuchong/rustup:stable 

WORKDIR /dks
COPY . .

# Windows (GNU)
RUN rustup toolchain install stable-x86_64-pc-windows-gnu && rustup target add x86_64-pc-windows-gnu
RUN apt-get update && apt-get -y install mingw-w64
# Windows - may be needed on WSL 2.0
RUN apt-get -y install build-essential clang cmake

# Linux (GNU)
RUN rustup toolchain install stable-x86_64-unknown-linux-gnu && rustup target add x86_64-unknown-linux-gnu

# BUILD
RUN mkdir -p /dks/build

RUN cargo build --release --target=x86_64-pc-windows-gnu
RUN cp target/x86_64-pc-windows-gnu/release/dks.exe /dks/build
RUN ls -lah target/x86_64-pc-windows-gnu/release

RUN cargo build --release --target=x86_64-unknown-linux-gnu
RUN cp target/x86_64-unknown-linux-gnu/release/dks /dks/build
RUN ls -lah target/x86_64-unknown-linux-gnu/release

RUN ls -lah /dks/build
