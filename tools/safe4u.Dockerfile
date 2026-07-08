FROM rust:1.83-bookworm

ENV DEBIAN_FRONTEND=noninteractive \
    PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1

RUN apt-get update \
    && apt-get install --yes --no-install-recommends \
        build-essential \
        ca-certificates \
        clang \
        cmake \
        pkg-config \
        python3 \
        python3-pip \
        python3-venv \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /opt/safe4u
COPY tools/safe4u/requirements.txt .
RUN python3 -m venv /opt/venv \
    && /opt/venv/bin/pip install --no-cache-dir --upgrade pip \
    && /opt/venv/bin/pip install --no-cache-dir -r requirements.txt

COPY tools/safe4u/ /opt/safe4u/
RUN chmod +x cargo-safe4u \
    && cargo build --release --manifest-path context_retriever/Cargo.toml

ENV PATH="/opt/venv/bin:/usr/local/cargo/bin:${PATH}"
WORKDIR /workspace
ENTRYPOINT ["/opt/safe4u/cargo-safe4u"]
CMD ["--help"]
