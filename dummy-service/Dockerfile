FROM rust:1.74.0-buster
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
EXPOSE 3000
CMD cargo run --release
