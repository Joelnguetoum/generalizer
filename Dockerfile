# ---------- Stage 1: Build ----------
#FROM rust:1.92.0 as builder
FROM rust:latest AS builder

WORKDIR /app

# System deps sometimes needed for crates
RUN apt-get update && apt-get -y install curl \
    ca-certificates \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*


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
        zip \
        nano \
        less \
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
RUN python3 -m pip install maude csvkit

RUN useradd -m fm
WORKDIR /home/fm/generalizer



COPY --from=builder /app ./

RUN mkdir "Executable"
RUN cp /home/fm/generalizer/target/release/generalizer "Executable"
RUN zip -r generalizer_sources.zip src readme Cargo.lock Cargo.toml Dockerfile LICENCE.txt README.txt
RUN rm Cargo.lock Cargo.toml README.md
RUN rm -r readme src target
RUN mv README_FM.md README.md

RUN chown -R fm:fm /home/fm

USER fm
CMD ["/bin/bash"]