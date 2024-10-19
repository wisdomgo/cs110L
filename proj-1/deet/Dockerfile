FROM ubuntu:18.04

RUN apt-get update && \
    apt-get install -y build-essential make curl strace gdb

# Install Rust. Don't use rustup, so we can install for all users (not just the
# root user)
RUN curl --proto '=https' --tlsv1.2 -sSf \
    https://sh.rustup.rs | sh -s -- -y

# Make .cargo writable by any user (so we can run the container as an
# unprivileged user)
RUN mkdir /.cargo && chmod 777 /.cargo

WORKDIR /deet