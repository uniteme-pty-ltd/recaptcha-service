# recaptcha-service

## Prerequisites

1. Install [Docker](https://www.docker.com/) by following it's [installation instructions](https://www.docker.com/products/docker-desktop/).

2. Install [Watch](https://watchexec.github.io/#cargo-watch) by running `cargo install cargo-watch`.

## During Development

When developing, ensure `cargo watch --clear -x fmt` is running in a terminal, where:
- `watch` will detect any changes in the repo and refresh commands.
- `--clear` will clear the console output each time changes are detected.
- `fmt` will format your Rust code to comply with standards and ensure unformatted code is not committed.

To launch the application run `docker compose up --build`, this will:
- expose the API via `http://localhos:<port>`

The individual ports uniquely chosen by Docker can be found through the Docker interface.