# 1.56.1 as of now (2021-11-14)
FROM liuchong/rustup:stable 

WORKDIR /dks
COPY . .

# Windows (GNU)
# ("build-essential", "clang", "cmake" may be needed on WSL 2.0)
RUN rustup toolchain install stable-x86_64-pc-windows-gnu && \
    rustup target add x86_64-pc-windows-gnu
RUN apt-get update && apt-get -y install mingw-w64 build-essential clang cmake

# BUILD
RUN mkdir -p /dks/build
RUN ls -lah

RUN cargo install --force cargo-strip
RUN cargo build --release --target=x86_64-pc-windows-gnu
RUN cargo strip --target x86_64-pc-windows-gnu

RUN mv target/x86_64-pc-windows-gnu/release/dks.exe .
RUN tar -cvzf dks_windows_x86_64.tar.gz dks.exe license readme.md
RUN cp dks_windows_x86_64.tar.gz /dks/build