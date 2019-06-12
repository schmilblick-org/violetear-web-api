FROM fedora AS builder

WORKDIR /usr/src/app

RUN dnf install -y cargo rust sqlite-devel sqlite-libs

# RUN curl https://sh.rustup.rs -sSf > rustup.sh
# RUN chmod u+x rustup.sh
# RUN ./rustup.sh -y --default-toolchain stable
# RUN rm rustup.sh

COPY . .
RUN cargo build --release

FROM fedora

# We'll likely need to add SSL root certificates
RUN dnf install -y sqlite-libs

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/app/target/release/web-api .

ENV PORT 5000

CMD ["web-api"]

EXPOSE 5000
