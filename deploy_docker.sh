#!/bin/bash

mkdir -p deploy_rs_docker
mkdir -p deploy_rs_docker/src

cat deploy.rs | grep -v '//!' | sed -n '1!p' > ./deploy_rs_docker/src/main.rs

cat <<EOF > ./deploy_rs_docker/Dockerfile
FROM rustlang/rust:nightly-slim

WORKDIR /usr/deploy_rs_docker

RUN mkdir -p src

COPY ./src/ /usr/deploy_rs_docker/src
COPY ./Cargo.toml /usr/deploy_rs_docker

VOLUME ["/usr/local/cargo"]

RUN apt-get update
RUN apt-get install pkg-config -y
RUN apt-get install libssl-dev -y

RUN cargo build --release

RUN mkdir -p /usr/mount
EOF

cat <<EOF > ./deploy_rs_docker/Cargo.toml
[package]
name = "deploy"
version = "0.1.0"
authors = ["Markus Reiter <me@reitermark.us>"]

EOF

cat deploy.rs | grep //! | sed -n '1!p' | sed -e '$d' | cut -c5- >> ./deploy_rs_docker/Cargo.toml

cd ./deploy_rs_docker

docker build -t deploy_rs_docker .

docker run -v "/$PWD/out:/usr/mount" deploy_rs_docker bash -c "cp /usr/deploy_rs_docker/target/release/deploy /usr/mount"

./deploy_rs_docker/out/deploy
