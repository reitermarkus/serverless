FROM openfaas/of-watchdog:0.7.6 as watchdog

FROM ekidd/rust-musl-builder:1.41.0 as builder

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

# Abuse the `ADDITIONAL_PACKAGE` arg to switch between debug and release builds.
ARG ADDITIONAL_PACKAGE=release
ARG PROFILE="${ADDITIONAL_PACKAGE}"

# Pre-cache template dependencies.
RUN cargo build ${PROFILE:+--${PROFILE}} --manifest-path main/Cargo.toml

# Pre-cache function dependencies using dummy implementation.
COPY function/Cargo.toml function/Cargo.toml
RUN cargo build ${PROFILE:+--${PROFILE}} --manifest-path main/Cargo.toml \
 && rm target/x86_64-unknown-linux-musl/${PROFILE:-debug}/main* \
 && rm target/x86_64-unknown-linux-musl/${PROFILE:-debug}/deps/libhandler* \
 && rm -r function

# Compile function function using actual implementation.
COPY function function
RUN cargo build ${PROFILE:+--${PROFILE}} --manifest-path main/Cargo.toml \
 && mv target/x86_64-unknown-linux-musl/${PROFILE:-debug}/main ../main

FROM scratch

COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog
COPY --from=builder /home/rust/main /usr/bin/main

ENV function_process=main
EXPOSE 8080

HEALTHCHECK --interval=2s CMD ["fwatchdog", "--run-healthcheck"]

ENV upstream_url="http://127.0.0.1:7878"
ENV mode="http"

CMD ["fwatchdog"]
