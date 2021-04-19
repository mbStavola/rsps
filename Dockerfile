FROM rust:1.51-buster

RUN mkdir /usr/src/app
WORKDIR /usr/src/app

RUN apt-get update
RUN apt-get install -y libunwind-dev

COPY ./Cargo.lock .
COPY ./Cargo.toml .

RUN mkdir .cargo
RUN cargo vendor > .cargo/config.toml

ADD ./src ./src

RUN cargo build