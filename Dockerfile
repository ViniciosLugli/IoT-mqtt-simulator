FROM rust:slim-bullseye

RUN apt-get update \
	&& apt-get install -y libssl-dev build-essential cmake \
	&& rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/workspace

COPY . .
