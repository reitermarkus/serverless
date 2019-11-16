FROM alpine as tempdir

RUN mkdir -p /tempdir

FROM node:alpine as builder

WORKDIR /home/app

COPY function/package.json .

# Pre-cache dependencies.
RUN yarn install --no-lockfile

COPY function /home/app

# Build function.
RUN yarn install --frozen-lockfile
RUN yarn build

FROM scratch

COPY --from=openfaas/of-watchdog:0.7.2 /fwatchdog /fwatchdog
COPY --from=tempdir /tempdir /tmp

WORKDIR /home/app

COPY --from=builder /home/app/dist public

# https://github.com/openfaas-incubator/of-watchdog/pull/87
# HEALTHCHECK --interval=2s CMD ["/fwatchdog", "healthcheck"]

ENV mode="static"

CMD ["/fwatchdog"]
