FROM rust:1.52-buster

RUN mkdir /usr/src/app
WORKDIR /usr/src/app

RUN apt-get update
RUN apt-get install -y libunwind-dev

COPY ./Cargo.lock .
COPY rsps/Cargo.toml .

RUN mkdir .cargo
RUN cargo vendor > .cargo/config.toml

ADD rsps/src ./src
COPY rsps/build.rs .

RUN cargo build