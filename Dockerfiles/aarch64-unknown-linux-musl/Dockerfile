FROM rustembedded/cross:aarch64-unknown-linux-musl-0.2.1
RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install --assume-yes musl-tools:arm64 && \
    apt-get install --assume-yes pkg-config:arm64 && \
    apt-get install --assume-yes libssl-dev:arm64 && \
    apt-get install --assume-yes libssl-dev && \
    apt-get install --assume-yes sqlite3:arm64 && \
    apt-get install --assume-yes libsqlite3-dev:arm64 \