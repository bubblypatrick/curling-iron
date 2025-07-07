FROM rust:1.88

WORKDIR /usr/src/curling-iron
COPY . .

RUN cargo install --path .

CMD ["curling-iron"]