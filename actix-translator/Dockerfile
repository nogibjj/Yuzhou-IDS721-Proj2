From rust:latest as builder
ENV APP rust-bert-rocket
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP
ENTRYPOINT ["/usr/local/bin/rust-bert-rocket"]
#export this actix web service to port 8080 and 0.0.0.0
EXPOSE 8080
CMD ["rust-bert-rocket"]