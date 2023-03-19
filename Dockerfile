# Build stage of the container
FROM rust:1.67.1-slim-buster as build
ARG PROFILE=release
WORKDIR /app

# Install required dependencies
RUN apt-get update
RUN apt-get -y upgrade
RUN apt-get -y install libpq-dev
RUN apt-get -y install pkg-config
RUN apt-get -y install libssl-dev

# Create dependency caching layer for fast recompilation with dummy main
RUN echo "fn main() {}" > /app/dummy.rs
COPY Cargo.toml /app/Cargo.toml
COPY Cargo.lock /app/Cargo.lock
RUN sed -i 's#^\[lib\]##' /app/Cargo.toml
RUN sed -i 's#^name = "recaptcha_client"##' /app/Cargo.toml
RUN sed -i 's#^path = "src\/client\/lib\.rs"##' /app/Cargo.toml
RUN sed -i 's#src/service/main.rs#dummy.rs#' /app/Cargo.toml
RUN cargo build --profile=$PROFILE

# Build the application
COPY Cargo.toml /app/Cargo.toml
COPY src /app/src
COPY examples /app/examples
RUN cargo build --profile=$PROFILE

# The final release stage of the container
FROM gcr.io/distroless/cc-debian11 as release

# Extract libpq for PostgreSQL Diesel connection
COPY --from=build /usr/lib/*-linux-gnu/libpq.so* /usr/lib/
COPY --from=build /usr/lib/*-linux-gnu/libgssapi_krb5.so* /usr/lib/
COPY --from=build /usr/lib/*-linux-gnu/libldap_r-2.4.so* /usr/lib/
COPY --from=build /usr/lib/*-linux-gnu/libkrb5.so* /usr/lib/
COPY --from=build /usr/lib/*-linux-gnu/libk5crypto.so* /usr/lib/
COPY --from=build /usr/lib/*-linux-gnu/libkrb5support.so* /usr/lib/
COPY --from=build /usr/lib/*-linux-gnu/liblber-2.4.so* /usr/lib/
COPY --from=build /usr/lib/*-linux-gnu/libsasl2.so* /usr/lib/
COPY --from=build /usr/lib/*-linux-gnu/libgnutls.so* /usr/lib/
COPY --from=build /usr/lib/*-linux-gnu/libp11-kit.so* /usr/lib/
COPY --from=build /usr/lib/*-linux-gnu/libidn2.so* /usr/lib/
COPY --from=build /usr/lib/*-linux-gnu/libunistring.so* /usr/lib/
COPY --from=build /usr/lib/*-linux-gnu/libtasn1.so* /usr/lib/
COPY --from=build /usr/lib/*-linux-gnu/libnettle.so* /usr/lib/
COPY --from=build /usr/lib/*-linux-gnu/libhogweed.so* /usr/lib/
COPY --from=build /usr/lib/*-linux-gnu/libgmp.so* /usr/lib/
COPY --from=build /usr/lib/*-linux-gnu/libffi.so* /usr/lib/
COPY --from=build /lib/*-linux-gnu/libcom_err.so* /lib/
COPY --from=build /lib/*-linux-gnu/libkeyutils.so* /lib/

# Extract compiled binary and environment variables
COPY --from=build /app/target/*/user_service /user_service

# Exponse the running port
EXPOSE 80

ENTRYPOINT ["/user_service"]
