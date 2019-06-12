FROM alpine AS builder

WORKDIR /usr/src/app

RUN apk --no-cache add sqlite-libs sqlite-dev

RUN curl https://sh.rustup.rs -sSf > rustup.sh
RUN chmod u+x rustup.sh
RUN ./rustup.sh -y --default-toolchain stable
RUN rm rustup.sh

COPY . .
RUN cargo build --release

FROM alpine

# We'll likely need to add SSL root certificates
RUN apk --no-cache add ca-certificates sqlite-libs

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/app/target/release/web-api .
CMD ["./web-api -l 0.0.0.0 -p 5000"]

EXPOSE 5000
