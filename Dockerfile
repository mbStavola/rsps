FROM rust:1.52-buster

RUN mkdir /usr/src/app
WORKDIR /usr/src/app

RUN apt-get update
RUN apt-get install -y libunwind-dev

COPY ./Cargo.lock .
COPY ./Cargo.toml .

RUN mkdir .cargo
RUN cargo vendor > .cargo/config.toml

ADD ./src ./src
COPY ./build.rs .

RUN cargo build