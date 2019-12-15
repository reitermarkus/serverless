FROM openfaas/of-watchdog:0.7.3 as watchdog

FROM rust:slim-buster as builder

WORKDIR /home/app

RUN mkdir -p function/src
RUN { \
  echo '[package]'; \
  echo 'name = "handler"'; \
  echo 'version = "0.1.0"'; \
  echo 'edition = "2018"'; \
  echo ''; \
  echo '[dependencies]'; \
  echo 'http = "*"'; \
  echo 'openfaas = { path = "../openfaas" }'; \
} > function/Cargo.toml
RUN { \
  echo 'pub async fn handle(_: http::Method, _: http::Uri, _: http::HeaderMap, _: String) -> Result<(http::StatusCode, String), Box<dyn std::error::Error + Send>> {'; \
  echo '  Ok((http::StatusCode::OK, "".to_string()))'; \
  echo '}'; \
} > function/src/lib.rs

COPY openfaas openfaas
COPY main main
COPY Cargo.toml Cargo.toml

# Pre-cache template dependencies.
RUN cargo build --release --manifest-path main/Cargo.toml

# Pre-cache function dependencies using dummy implementation.
COPY function/Cargo.toml function/Cargo.toml
RUN cargo build --release --manifest-path main/Cargo.toml

# Compile function function using actual implementation.
RUN rm -r function
COPY function function

RUN rm target/release/main*
RUN rm target/release/deps/libhandler*

RUN cargo build --release --manifest-path main/Cargo.toml

FROM debian:buster-slim as ship

COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog
RUN chmod +x /usr/bin/fwatchdog

ARG ADDITIONAL_PACKAGE
RUN apt-get update -qy \
    && apt-get install -qy --no-install-recommends ${ADDITIONAL_PACKAGE} \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN addgroup --system app \
    && adduser --system --ingroup app app

RUN chown app:app -R /home/app

USER app

WORKDIR /home/app
COPY --from=builder /home/app/target/release/main /home/app/

ENV function_process=./main
EXPOSE 8080

HEALTHCHECK --interval=2s CMD ["fwatchdog", "--run-healthcheck"]

ENV upstream_url="http://127.0.0.1:7878"
ENV mode="http"

CMD ["fwatchdog"]
