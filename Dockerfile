# This file is a template, and might need editing before it works on your project.
FROM rust AS builder

WORKDIR /usr/src/app

COPY . .
RUN cargo build --release

FROM alpine

# We'll likely need to add SSL root certificates
RUN apk --no-cache add ca-certificates

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/app/target/release/web-api .
CMD ["./web-api -l 0.0.0.0 -p 5000"]

EXPOSE 5000