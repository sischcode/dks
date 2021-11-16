# 1.56.1 as of now (2021-11-14)
FROM liuchong/rustup:stable 

WORKDIR /dks
COPY . .

# Linux (GNU)
RUN rustup toolchain install stable-x86_64-unknown-linux-gnu && \
    rustup target add x86_64-unknown-linux-gnu

# BUILD
RUN mkdir -p /dks/build
RUN ls -lah
RUN cargo install --force cargo-strip
RUN cargo build --release --target=x86_64-unknown-linux-gnu
RUN cargo strip --target x86_64-unknown-linux-gnu
RUN mv target/x86_64-unknown-linux-gnu/release/dks .
RUN tar -cvzf dks_linux_x86_64.tar.gz dks license readme.md
RUN cp dks_linux_x86_64.tar.gz /dks/build