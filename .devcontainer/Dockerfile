FROM ubuntu:24.04
ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=Etc/UTC
ARG USER_ID=1000
ARG GROUP_ID=1000

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    ca-certificates \
    software-properties-common \
    ninja-build \
    sudo \
    git \
    wget \
    tmux \
    cmake \
    unzip \
    libz3-dev \
    libgoogle-perftools-dev \
    libsqlite3-dev \
# Needed for falcon-capstone-0.5.2
    libcapstone-dev \ 
# Needed for stainless_ffmpeg-0.5.4
    libavutil-dev \
    libavformat-dev \
    libavfilter-dev \
    libavdevice-dev \
# Needed for mmdeploy-0.9.0
    libopencv-dev \
# Needed for mmap-loader-0.5.3
    protobuf-compiler \
    && wget https://apt.llvm.org/llvm.sh \
    && chmod +x llvm.sh \
    && sudo ./llvm.sh 20 \
    && rm llvm.sh \
    && apt install -y llvm-20 llvm-20-dev clang-20 \
    && ln -s /usr/bin/clang-20 /usr/bin/clang \
    && ln -s /usr/bin/clang++-20 /usr/bin/clang++ \
# Clean up
    && rm -rf /var/lib/apt/lists/*

# Create non-root user 'user' with the same UID and GID as the host user
RUN if getent passwd ${USER_ID} > /dev/null 2>&1; then \
        EXISTING_USER=$(getent passwd ${USER_ID} | cut -d: -f1) && \
        if [ "${EXISTING_USER}" != "user" ]; then \
            usermod -l user -d /home/user -m ${EXISTING_USER}; \
        fi; \
    elif ! getent group ${GROUP_ID} > /dev/null 2>&1; then \
        groupadd -g ${GROUP_ID} user && \
        useradd -m -u ${USER_ID} -g user user; \
    else \
        EXISTING_GROUP=$(getent group ${GROUP_ID} | cut -d: -f1) && \
        useradd -m -u ${USER_ID} -g ${EXISTING_GROUP} user; \
    fi && \
    echo "user:docker" | chpasswd && \
    usermod -aG sudo user && \
    echo "user ALL=(ALL) NOPASSWD:ALL" >> tee /etc/sudoers.d/user


# Install Rust 1.87.0 as 'user'
USER user
WORKDIR /home/user
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y --default-toolchain nightly-2025-03-28
ENV PATH="/home/user/.cargo/bin:${PATH}"
RUN rustup component add rust-src rustc-dev llvm-tools rust-analyzer
RUN rustup target add x86_64-unknown-linux-gnu
RUN curl -LsSf https://astral.sh/uv/0.10.2/install.sh | sh