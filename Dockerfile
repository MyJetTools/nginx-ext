FROM amigin/nginx:1.0.0
COPY ./target/release/ca-api ./target/release/ca-api
COPY ./wwwroot ./wwwroot 
ENTRYPOINT ["./target/release/ca-api"]