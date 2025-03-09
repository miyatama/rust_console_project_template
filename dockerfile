FROM rust:1.84 AS builder
WORKDIR /home/rust
COPY . .
RUN cargo build --release

FROM alpine:latest
WORKDIR /app_name
COPY --from=builder /home/rust/target/release/ui . 
# EXPOSE 8080
ENTRYPOINT [ "./ui" ]
CMD [ "--help" ]
