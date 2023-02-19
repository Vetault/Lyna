# We want to use a nightly-only compiler option called build-std:
# It basically builds std as a dependency of your project and allows LTO to kick in
# and remove unused parts of std! This makes the binary smaller. build-std requires
# specifying --target manually every time, so we use a variable for the target we build for.
#
# This will compile for x86_64 by default.
# By using ARG, we can do `docker build --build-arg aarch64-unknown-linux-musl` to for example
# cross compile to aarch64.
ARG RUST_TARGET="x86_64-unknown-linux-musl"
# For downloading a cross compiler, we also need to know the musl target. This defaults to x86_64.
ARG MUSL_TARGET="x86_64-linux-musl"

# The binary must be built in an Alpine linux environment
FROM docker.io/library/alpine:edge AS builder
ARG RUST_TARGET
ARG MUSL_TARGET

# Update all installed packages and install a GCC toolchain and a nightly Rust compiler.
RUN apk upgrade && \
    apk add curl gcc musl-dev && \
    curl -sSf https://sh.rustup.rs | sh -s -- --profile minimal --default-toolchain nightly --component rust-src -y

# Set up a cross compiler if needed.
# This also configures rustc to link with the cross compiler and use the nightly-only
# build-std feature.
RUN source $HOME/.cargo/env && \
    mkdir -p /app/.cargo && \
    if [ "$RUST_TARGET" != $(rustup target list --installed) ]; then \
        rustup target add $RUST_TARGET && \
        curl -L "https://musl.cc/$MUSL_TARGET-cross.tgz" -o /toolchain.tgz && \
        tar xf toolchain.tgz && \
        ln -s "/$MUSL_TARGET-cross/bin/$MUSL_TARGET-gcc" "/usr/bin/$MUSL_TARGET-gcc" && \
        ln -s "/$MUSL_TARGET-cross/bin/$MUSL_TARGET-ld" "/usr/bin/$MUSL_TARGET-ld" && \
        ln -s "/$MUSL_TARGET-cross/bin/$MUSL_TARGET-strip" "/usr/bin/actual-strip" && \
        GCC_VERSION=$($MUSL_TARGET-gcc --version | grep gcc | awk '{print $3}') && \
        echo -e "\
[build]\n\
rustflags = [\"-L\", \"native=/$MUSL_TARGET-cross/$MUSL_TARGET/lib\", \"-L\", \"native=/$MUSL_TARGET-cross/lib/gcc/$MUSL_TARGET/$GCC_VERSION/\", \"-l\", \"static=gcc\", \"-Z\", \"gcc-ld=lld\"]\n\
[target.$RUST_TARGET]\n\
linker = \"$MUSL_TARGET-gcc\"\n\
[unstable]\n\
build-std = [\"std\", \"panic_abort\"]\n\
" > /app/.cargo/config; \
    else \
        echo "skipping toolchain as we are native" && \
        echo -e "\
[build]\n\
rustflags = [\"-L\", \"native=/usr/lib\"]\n\
[unstable]\n\
build-std = [\"std\", \"panic_abort\"]\n\
" > /app/.cargo/config && \
        ln -s /usr/bin/strip /usr/bin/actual-strip; \
    fi

WORKDIR /app

# Copy our dependency file
COPY Cargo.toml Cargo.lock ./

# Run a build with an empty main.rs file, basically a fake source.
# Why? Because this allows Docker to cache all dependencies, and it
# won't recompile all of them if you rebuild your image.
RUN mkdir src/
RUN echo 'fn main() {}' > ./src/main.rs
RUN source $HOME/.cargo/env && \
    cargo build --release --target="$RUST_TARGET"

# Now, delete the fake source and copy in the actual source. This allows us to
# have a previous compilation step for compiling the dependencies, while being
# able to only copy in and compile the binary itself when something in the
# source changes.
#
# This is very important. If we just copy in the source after copying in the
# Cargo.lock and Cargo.toml, then every time the source changes the dependencies
# would have to be re-downloaded and re-compiled.
#
# Also, remove the artifacts of building the binaries.
RUN rm -f target/$RUST_TARGET/release/deps/bot*
COPY ./src ./src

# If you depend on any other files (e.g. database schemas), COPY them here too
# COPY ./migrations ./migrations

# Compile our actual source and strip the resulting binary.
RUN source $HOME/.cargo/env && \
    cargo build --release --target="$RUST_TARGET" && \
    cp target/$RUST_TARGET/release/bot /bot && \
    strip /bot

# We want to place the binary in an empty container, hence scratch.
# It is a fully statically linked binary, so it will work just fine without
# having a full OS like Alpine or Debian around it.
FROM scratch

# Copy it from the build container.
COPY --from=builder /bot /bot

# And run it when the user starts the container.
CMD [ "./bot" ]
