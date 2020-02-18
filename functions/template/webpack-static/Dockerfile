FROM openfaas/of-watchdog:0.7.6 as watchdog

FROM node:13.8.0-alpine as builder

WORKDIR /home/app

COPY function/package.json .

# Pre-cache dependencies.
RUN yarn install --no-lockfile

COPY function /home/app

# Build function.
RUN yarn install --frozen-lockfile

# Abuse the `ADDITIONAL_PACKAGE` arg to switch between development and production builds.
ARG ADDITIONAL_PACKAGE=production
ARG NODE_ENV="${ADDITIONAL_PACKAGE}"

RUN yarn build

FROM scratch

WORKDIR /home/app

COPY --from=watchdog /fwatchdog /fwatchdog
COPY --from=builder /home/app/dist public

HEALTHCHECK --interval=2s CMD ["/fwatchdog", "--run-healthcheck"]

ENV mode="static"

CMD ["/fwatchdog"]
