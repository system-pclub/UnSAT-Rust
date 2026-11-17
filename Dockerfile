FROM ubuntu:24.04

ARG DEBIAN_FRONTEND=noninteractive
ARG USER_ID=1000
ARG GROUP_ID=1000
ENV TZ=Etc/UTC

RUN apt-get update && apt-get install -y --no-install-recommends \
        bubblewrap \
        build-essential \
        ca-certificates \
        cmake \
        curl \
        git \
        libavdevice-dev \
        libavfilter-dev \
        libavformat-dev \
        libavutil-dev \
        libcapstone-dev \
        libgoogle-perftools-dev \
        libopencv-dev \
        libsqlite3-dev \
        libssl-dev \
        libz3-dev \
        ninja-build \
        pkg-config \
        protobuf-compiler \
        python3 \
        python3-venv \
        software-properties-common \
        sudo \
        tmux \
        unzip \
        wget \
        zlib1g-dev \
    && wget -q https://apt.llvm.org/llvm.sh \
    && chmod +x llvm.sh \
    && ./llvm.sh 20 \
    && rm llvm.sh \
    && apt-get install -y --no-install-recommends llvm-20 llvm-20-dev clang-20 \
    && ln -sf /usr/bin/clang-20 /usr/local/bin/clang \
    && ln -sf /usr/bin/clang++-20 /usr/local/bin/clang++ \
    && rm -rf /var/lib/apt/lists/*

RUN if ! getent group "${GROUP_ID}" >/dev/null; then groupadd --gid "${GROUP_ID}" user; fi \
    && group_name="$(getent group "${GROUP_ID}" | cut -d: -f1)" \
    && if ! getent passwd "${USER_ID}" >/dev/null; then \
           useradd --create-home --uid "${USER_ID}" --gid "$group_name" user; \
       else \
           existing_user="$(getent passwd "${USER_ID}" | cut -d: -f1)"; \
           usermod --login user --home /home/user --move-home "$existing_user"; \
       fi \
    && usermod -aG sudo user \
    && printf 'user ALL=(ALL) NOPASSWD:ALL\n' >/etc/sudoers.d/user

RUN curl -LsSf https://astral.sh/uv/0.10.2/install.sh | env UV_INSTALL_DIR=/usr/local/bin sh

COPY tools/x/pyproject.toml tools/x/uv.lock /opt/x/
RUN uv sync --project /opt/x --frozen --no-install-project --extra test \
    && chown -R user:$(id -gn user) /opt/x

USER user
ENV PATH="/opt/x/.venv/bin:/home/user/.cargo/bin:${PATH}"
WORKDIR /home/user

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly-2025-03-28 \
    && rustup component add rust-src rustc-dev llvm-tools rust-analyzer \
    && rustup target add x86_64-unknown-linux-gnu

# Keep benchmark Cargo invocations pinned without downloading a toolchain per run.
RUN rustup toolchain install 1.87.0 --profile minimal

WORKDIR /unsat-rust
