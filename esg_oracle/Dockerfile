FROM rust:1.56 as builder

WORKDIR /esg_oracle
COPY . .

RUN cargo build --release --bin esg_oracle

FROM debian:buster-slim

ARG APP=/usr/src/app

# Some sensible packages to use once we deploy this
RUN apt-get update --no-install-recommends \
    && apt-get install -y --no-install-recommends ca-certificates=20200601~deb10u2 tzdata=2021a-0+deb10u3 \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /esg_oracle/target/release/esg_oracle ${APP}/esg_oracle

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}
ENV RUST_LOG=info

CMD ["./esg_oracle"]