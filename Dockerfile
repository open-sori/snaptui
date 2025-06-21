FROM docker.io/alpine:3.22 AS builder

ARG SNAPTUI_VERSION
ARG TARGETARCH

RUN apk add --update --no-cache curl && \
    case ${TARGETARCH} in \
        "amd64") export ARCH_SUFFIX="x86_64-unknown-linux-musl" ;; \
        "arm64") export ARCH_SUFFIX="aarch64-unknown-linux-musl" ;; \
        *) echo "Unsupported architecture: ${TARGETARCH}" >&2; exit 1 ;; \
    esac && \
    curl \
        --silent \
        --location \
        --fail \
        --request GET "https://github.com/open-sori/snaptui/releases/download/${SNAPTUI_VERSION}/snaptui-${ARCH_SUFFIX}" \
        --output /tmp/snaptui && \
    chmod +x /tmp/snaptui && \
    ls /bin

FROM scratch

ARG SNAPTUI_VERSION
ARG CREATED_DATE

# Ref from https://github.com/opencontainers/image-spec/blob/main/annotations.md
LABEL org.opencontainers.image.created="$CREATED_DATE"
LABEL org.opencontainers.image.authors="thibault@open-sori.dev"
LABEL org.opencontainers.image.url="https://github.com/orgs/open-sori/packages/container/package/snaptui"
LABEL org.opencontainers.image.documentation="https://snaptui.open-sori.dev"
LABEL org.opencontainers.image.source="https://github.com/open-sori/snaptui"
LABEL org.opencontainers.image.version="$SNAPTUI_VERSION"
LABEL org.opencontainers.image.revision="$SNAPTUI_VERSION"
LABEL org.opencontainers.image.vendor="open-sori"
LABEL org.opencontainers.image.licenses="GPL-3.0-or-later"
LABEL org.opencontainers.image.ref.name="snaptui"
LABEL org.opencontainers.image.title="snaptui"
LABEL org.opencontainers.image.description="snapcast ctl binary docker image"
LABEL org.opencontainers.image.base.name="ghcr.io/open-sori/snaptui:${SNAPTUI_VERSION}"

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

COPY --from=builder /tmp/snaptui /bin/snaptui

ENV SNAPSERVER_HOST="127.0.0.1"
ENV SNAPSERVER_PORT="1780"

ENTRYPOINT ["/bin/snaptui"]