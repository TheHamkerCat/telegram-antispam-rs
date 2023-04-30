FROM rust as rust-builder

WORKDIR /usr/src/app
COPY ./Cargo.toml .
COPY ./Cargo.lock .

# For caching dependencies and avoid rebuilding them
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs
RUN cargo build --release
RUN rm -rf ./src
COPY ./src ./src
RUN touch -a -m ./src/main.rs
RUN cargo build --release

FROM debian:buster-slim
COPY --from=rust-builder /usr/src/app/target/release/tg_antispam_rs /usr/local/bin/
WORKDIR /usr/local/bin

RUN apt-get update
RUN apt-get install wget -y
RUN wget "https://huggingface.co/datasets/thehamkercat/telegram-spam-ham/raw/main/dataset.csv"

CMD ["tg_antispam_rs"]
