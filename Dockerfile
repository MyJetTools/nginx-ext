FROM goacme/lego as builder

FROM nginx:latest
COPY --from=builder /lego /lego
COPY ./target/release/nginx-ext ./target/release/nginx-ext
ENTRYPOINT ["./target/release/nginx-ext"]
