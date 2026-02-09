# ---------- Stage 1: Build ----------
#FROM rust:1.92.0 as builder
FROM rust:latest as builder

WORKDIR /app


# --- Make Cargo networking more reliable ---
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
ENV CARGO_NET_RETRY=10
ENV CARGO_HTTP_TIMEOUT=600

# System deps sometimes needed for crates
RUN apt-get update && apt-get -y install curl \
    ca-certificates \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*


# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy full project
COPY . .

# Build real binary
RUN cargo build --release


# ---------- Stage 2: Runtime ----------
FROM ubuntu:24.04

#ENV DEBIAN_FRONTEND=noninteractive

# Install runtime dependencies + Python + pip
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        bash \
        python3 \
        python3-pip \
        python3-venv \
    && rm -rf /var/lib/apt/lists/*

# create a virtual environment for Python deps
ENV VIRTUAL_ENV=/opt/venv
RUN python3 -m venv $VIRTUAL_ENV
ENV PATH="$VIRTUAL_ENV/bin:$PATH"

# Install Maude Python package
RUN python3 -m pip install maude

RUN useradd -m fm
WORKDIR /home/fm/generalizer

COPY --from=builder /app ./

RUN cp /home/fm/generalizer/target/release/generalizer "Benchmark Composition" && \
    cp /home/fm/generalizer/target/release/generalizer Examples && \
    chown -R fm:fm /home/fm

USER fm
CMD ["/bin/bash"]