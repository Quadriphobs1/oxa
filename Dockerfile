FROM  ubuntu:focal AS builder

WORKDIR /root/

# Copy the contents of this repository to the container
COPY . .

# ------------------------------------------------------------------------------

RUN echo "## Start building" \
    && echo "## Update and install packages" \
    && apt-get -qq -y update \
    && apt-get -qq install -y --no-install-recommends \
        binutils \
        build-essential \
        ca-certificates \
        file \
        git \
        curl \
        openssl \
        lld \
        clang \
        zip \
    && echo "## Done"

FROM rust:1.65.0 AS runner

WORKDIR /home/

ENV PATH="/root/.cargo/bin:$PATH"
ENV PATH="/root/.cargo/bin:$PATH"
ENV RUSTFLAGS="-Cinstrument-coverage"
ENV LLVM_PROFILE_FILE="oxa-%p-%m.profraw"

RUN echo "Installing required deps"

RUN rustup install nightly

RUN rustup default nightly

RUN rustup component add llvm-tools-preview

RUN cargo install grcov

FROM runner AS coverage

# Copy the contents of this repository to the container
COPY . .

CMD ["./scripts/coverage.sh"]