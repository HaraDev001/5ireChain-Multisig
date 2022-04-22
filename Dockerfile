# syntax=docker/dockerfile:1.3-labs
# hadolint ignore=DL3007
FROM 5irechain/builder:latest as cargo
WORKDIR /5ire

FROM cargo AS builder

COPY . /5ire
RUN <<EOF
    git config --global url.git@github.com:.insteadOf https://github.com/
    stat ~/.ssh > /dev/null 2>&1 || mkdir -m 0600 ~/.ssh
    ssh-keyscan github.com >> ~/.ssh/known_hosts
EOF

ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
RUN --mount=type=ssh cargo --config net.git-fetch-with-cli=true -Z unstable-options build --release \
    && cp /5ire/target/release/node-5ire /5ire/node-5ire

FROM ubuntu:22.04 as runtime

EXPOSE 9944 9615 9933 30333
WORKDIR /5ire

COPY --from=builder /5ire/node-5ire /5ire/node-5ire
RUN <<EOF
    apt update
    apt install -y --no-install-recommends ca-certificates
    chmod +x /5ire/node-5ire
    mkdir -p /5ire/chains/testnet/
EOF

COPY ./chains /5ire/chains

RUN <<EOF
    ##############################
    ##### Verify the binary #####
    ldd /5ire/node-5ire
    /5ire/node-5ire --version
    ##############################
EOF

VOLUME ["/data"]

ENTRYPOINT ["/5ire/node-5ire", "--chain", "/5ire/chains/testnet/5fire.json", "-d", "/data", "--prometheus-external", "--ws-external", "--rpc-external", "--rpc-cors", "all", "--rpc-methods=unsafe"]