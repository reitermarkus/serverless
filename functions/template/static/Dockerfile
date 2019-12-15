FROM openfaas/of-watchdog:0.7.3 as watchdog

FROM debian:stretch-slim as ship

COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog
RUN chmod +x /usr/bin/fwatchdog

# Create a non-root user
RUN addgroup --system app \
    && adduser --system --ingroup app app

RUN chown app:app -R /home/app

USER app

WORKDIR /home/app

COPY function public

HEALTHCHECK --interval=2s CMD ["/fwatchdog", "--run-healthcheck"]

ENV mode="static"

CMD ["fwatchdog"]
