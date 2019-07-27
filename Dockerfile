FROM %deploy_image%

WORKDIR /usr/local/bin

COPY . .

RUN rm -rf target
COPY target/release/web-api .

ENV PORT 5000

CMD diesel setup && diesel migration run && web-api

EXPOSE 5000
