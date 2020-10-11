FROM rust:1.43.1 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/data-api
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10
WORKDIR /usr/local/bin
COPY --from=build /usr/local/cargo/bin/data-api .
COPY --from=build /usr/src/data-api/src/data ./src/data
CMD ["data-api"]
