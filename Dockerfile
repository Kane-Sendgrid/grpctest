FROM ubuntu:latest

USER root
ENV USER root

# Install package dependencies.
RUN apt-get update \
    && apt-get install -y \
    apt-utils cmake build-essential golang python \
    curl \
    gcc \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl https://sh.rustup.rs -sSf > /tmp/rustup-init.sh \
    && chmod +x /tmp/rustup-init.sh \
    && sh /tmp/rustup-init.sh -y \
    && rm -rf /tmp/rustup-init.sh
ENV PATH "$PATH:~/.cargo/bin"

# Update the local crate index
RUN ~/.cargo/bin/cargo search

# Install nightly rust.
RUN ~/.cargo/bin/rustup install nightly
RUN ~/.cargo/bin/rustup default nightly

# Initialize a dummy project so that we can pre-download the latest versions of
# the most popular crates, by building and destroy an empty project.
RUN mkdir /tmp/dummy-crate
WORKDIR /tmp/dummy-crate
RUN ~/.cargo/bin/cargo init \
    && echo "argparse = '*'" >> Cargo.toml \
    && echo "env_logger = '*'" >> Cargo.toml \
    && echo "hyper = '*'" >> Cargo.toml \
    && echo "itertools = '*'" >> Cargo.toml \
    && echo "log = '*'" >> Cargo.toml \
    && echo "matches = '*'" >> Cargo.toml \
    && echo "num = '*'" >> Cargo.toml \
    && echo "rand = '*'" >> Cargo.toml \
    && echo "regex = '*'" >> Cargo.toml \
    && echo "semver = '*'" >> Cargo.toml \
    && echo "tempdir = '*'" >> Cargo.toml \
    && echo "time = '*'" >> Cargo.toml \
    && echo "tokio-core = '*'" >> Cargo.toml \
    && echo "futures = '*'" >> Cargo.toml \
    && echo "chrono = '*'" >> Cargo.toml \
    && echo "grpcio = '*'" >> Cargo.toml \
    && echo "clap = '*'" >> Cargo.toml \
    && echo "url = '*'" >> Cargo.toml \
    && ~/.cargo/bin/cargo build \
    && rm -rf /tmp/dummy-crate
RUN ~/.cargo/bin/cargo install protobuf
RUN ~/.cargo/bin/cargo install grpcio-compiler
RUN apt-get install -y python
