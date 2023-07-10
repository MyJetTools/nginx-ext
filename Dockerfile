FROM rust:slim
COPY ./target/release/ca-api ./target/release/ca-api
COPY ./wwwroot ./wwwroot 
ENTRYPOINT ["./target/release/ca-api"]