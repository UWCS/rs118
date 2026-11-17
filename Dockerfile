FROM rust:1-slim-trixie AS build

RUN apt-get update && apt-get install -y --no-install-recommends patch build-essential pkg-config
RUN cargo install mdbook@=0.5.1 mdbook-toc@=0.15.3 mdbook-katex@=0.10.0-alpha

WORKDIR /site
COPY book.toml .
COPY src/ ./src/
RUN mdbook build

FROM nginx:stable-alpine
COPY nginx.conf /etc/nginx/nginx.conf.d/default.conf
COPY --from=build /site/book /usr/share/nginx/html/
EXPOSE 80