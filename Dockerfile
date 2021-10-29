# Stage One -- Setup dependencies
FROM rust:1.56-buster AS dependencies

RUN apt-get update &&\
  apt-get install --no-install-recommends -y\
  cmake\
  musl-tools\
  musl-dev\
  pkgconf\
  inotify-tools\
  libsasl2-dev\
  libmongoc-1.0-0\
  libbson-1.0 &&\
  apt-get autoremove && apt-get clean &&\
  rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /build

# Make a simple placeholder so cargo can build depencies
RUN mkdir -p ./src/bin && echo "use api::*; fn main() { }" > ./src/bin/server.rs
RUN mkdir -p ./src && echo "" > ./src/lib.rs
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release --target x86_64-unknown-linux-musl

# Stage Two -- Build Project
FROM dependencies AS builder

COPY . .
RUN touch ./src/bin/server.rs && touch ./src/lib.rs
RUN cargo build --release --target x86_64-unknown-linux-musl


# Stage Three -- Copy Output to empty container
FROM scratch
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/server .
CMD ["./server"]
