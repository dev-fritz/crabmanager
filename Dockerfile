FROM rust:latest
WORKDIR /app
COPY . /app/
RUN cargo build --verbose --release
EXPOSE 8080
CMD ["cargo", "run", "--release"]