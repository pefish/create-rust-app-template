FROM pefish/ubuntu-rust:v1.82_2 AS builder
WORKDIR /app
COPY ./ ./
RUN make

FROM ubuntu:22.04
WORKDIR /app
COPY --from=builder /app/target/release/ /app/bin/
CMD ["/app/bin/app-name"]

# docker build --progress=plain -t pefish/app-name:v0.0.1 .
# docker run -ti --env-file ./.env --name app-name pefish/app-name:v0.0.1
