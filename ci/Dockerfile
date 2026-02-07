FROM alpine
COPY ./target/x86_64-unknown-linux-musl/debug/ricochet /usr/local/bin/ricochet
RUN apk add --no-cache libc6-compat
EXPOSE 8081
ENV HOST=0.0.0.0
CMD ["/usr/local/bin/ricochet"]