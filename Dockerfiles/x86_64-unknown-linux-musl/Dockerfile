FROM rustembedded/cross:x86_64-unknown-linux-musl-0.2.1
RUN apt-get update && \
    apt-get install --assume-yes musl-tools && \
    apt-get install --assume-yes pkg-config && \
    apt-get install --assume-yes libssl-dev &&  \
    apt-get install --assume-yes sqlite3 &&  \
    apt-get install --assume-yes libsqlite3-dev \