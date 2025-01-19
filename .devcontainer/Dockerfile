FROM rust:1.84.0-bookworm

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    libssl-dev \
    pkg-config \
    python3 \
    python3-pip \
    python3-venv \
    time \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
RUN python3 -m venv /app/venv
ENV PATH="/app/venv/bin:$PATH"

RUN python -m ensurepip --upgrade
RUN python -m pip install --upgrade pip
RUN pip3 install --upgrade maturin

RUN rustup component add rustfmt
RUN rustup component add clippy

