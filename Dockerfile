# Build stage of the container
FROM rust-build as build
ARG PROFILE=release
WORKDIR /app

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

# Final stage of the container
FROM rust-final as final

# Extract compiled binary and environment variables
COPY --from=build /app/target/*/recaptcha_service /recaptcha_service

# Exponse the running port
EXPOSE 80

HEALTHCHECK --interval=3s --timeout=3s --start-period=10s --retries=5 CMD [ "curl -f http://127.0.0.1/health || exit 1" ]
# HEALTHCHECK NONE

ENTRYPOINT ["/recaptcha_service"]
