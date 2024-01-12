# Sample configuration taken from leptos
# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-alpine as builder

#Install libc equivalent
RUN apk add --no-cache musl-dev
RUN apk add --no-cache pkgconfig
RUN apk add --no-cache libressl-dev

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY . .
WORKDIR /app/aep-schedule-website

# Build the app
RUN cargo leptos build --release -vv

FROM alpine as runner
# Copy the server binary to the /app directory
COPY --from=builder /app/aep-schedule-website/target/release/aep-schedule-website /app/
# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/aep-schedule-website/target/site /app/site

WORKDIR /app
# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:6942"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 6942
# Run the server
CMD ["/app/aep-schedule-website"]
