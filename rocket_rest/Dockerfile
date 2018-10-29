FROM rustlang/rust:nightly-slim

WORKDIR /usr/src/app

ADD /src /usr/src/app/src

COPY ./Cargo.lock /usr/src/app
COPY ./Cargo.toml /usr/src/app

EXPOSE 8000

VOLUME ["/usr/local/cargo"]

RUN cargo build --release

CMD ["cargo", "run", "--release"]
