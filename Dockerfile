FROM nginx:1.24.0
COPY ./target/release/nginx-ext ./target/release/nginx-ext
ENTRYPOINT ["./target/release/nginx-ext"]