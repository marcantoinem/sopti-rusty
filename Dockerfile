# Sample configuration taken from leptos
# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-alpine AS builder

RUN echo "http://dl-cdn.alpinelinux.org/alpine/edge/testing" >> /etc/apk/repositories

RUN apk update
RUN apk add --no-cache musl-dev pkgconfig openssl-dev openssl-libs-static cargo-leptos
RUN cargo install -f wasm-bindgen-cli --version 0.2.106

RUN wget -q https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64-musl \
    && chmod +x tailwindcss-linux-x64-musl \
    && mv tailwindcss-linux-x64-musl /usr/local/bin/tailwindcss

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY . .

# Build the app
RUN cargo leptos build --release -vv

FROM alpine AS runner
# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/server /app/
# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/alternance.csv /app/

WORKDIR /app
# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:6942"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 6942
# Run the server
CMD ["/app/server"]
