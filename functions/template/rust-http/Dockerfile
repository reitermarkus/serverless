FROM openfaas/of-watchdog:0.7.3 as watchdog

FROM ekidd/rust-musl-builder as builder

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
  echo 'pub async fn handle(_: http::Method, _: http::Uri, _: http::HeaderMap, _: String) -> Result<(http::StatusCode, String), Box<dyn std::error::Error + Send + Sync>> {'; \
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

RUN rm target/x86_64-unknown-linux-musl/release/main*
RUN rm target/x86_64-unknown-linux-musl/release/deps/libhandler*

# Compile function function using actual implementation.
RUN rm -r function
COPY function function

RUN cargo build --release --manifest-path main/Cargo.toml

FROM scratch

COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/main .

ENV function_process=./main
EXPOSE 8080

HEALTHCHECK --interval=2s CMD ["fwatchdog", "--run-healthcheck"]

ENV upstream_url="http://127.0.0.1:7878"
ENV mode="http"

CMD ["fwatchdog"]
