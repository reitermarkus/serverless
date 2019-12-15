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

COPY --from=openfaas/of-watchdog:0.7.3 /fwatchdog /fwatchdog

WORKDIR /home/app

COPY --from=builder /home/app/dist public

HEALTHCHECK --interval=2s CMD ["/fwatchdog", "--run-healthcheck"]

ENV mode="static"

CMD ["/fwatchdog"]
