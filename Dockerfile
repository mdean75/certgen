FROM room303.jfrog.io/docker-local/rust-builder:v1

RUN rustup target add x86_64-unknown-linux-gnu && apt install gcc-x86-64-linux-gnu

WORKDIR /app

COPY . .

CMD ["/usr/bin/make", "build-linux-x86"]
