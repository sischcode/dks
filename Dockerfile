# 1.56.1 as of now (2021-11-14)
FROM liuchong/rustup:stable 

WORKDIR /dks
COPY . .

# RUN chmod -R 755 /root/.cargo/bin

# Linux
RUN mkdir -p target/x86_64-unknown-linux-gnu && chmod -R 755 target/x86_64-unknown-linux-gnu
RUN rustup toolchain install stable-x86_64-unknown-linux-gnu && rustup target add x86_64-unknown-linux-gnu

# Windows (GNU)
RUN rustup toolchain install stable-x86_64-pc-windows-gnu && rustup target add x86_64-pc-windows-gnu
RUN apt-get update && apt-get -y install mingw-w64
# may be needed on WSL 2.0
RUN apt-get -y install build-essential clang cmake