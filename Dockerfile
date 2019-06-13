FROM %deploy_image%

WORKDIR /usr/local/bin

COPY target/release/web-api .

ENV PORT 5000

CMD ["web-api"]

EXPOSE 5000
