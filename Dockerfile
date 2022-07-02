FROM rustlang/rust:nightly-slim
RUN apt update; apt install -y libpq-dev

WORKDIR /code
COPY . .
RUN cargo build -r
RUN mv ./target/release/mango /mango
RUN rm -rf ./target

EXPOSE 8000
CMD /mango

LABEL org.opencontainers.image.source https://github.com/overlisted/mango
