FROM --platform=linux/x86_64 mcr.microsoft.com/devcontainers/base:debian

RUN apt update \
    && apt install -y pkgconf libclang-dev \
    && rm -rf /var/lib/apt/lists

ENTRYPOINT [ "sleep", "infinity" ]
