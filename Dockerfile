# BUILD
FROM rust:alpine AS build
WORKDIR /app
COPY . .
RUN cargo build --release

# RUN
FROM alpine:latest AS run
RUN apk add --no-cache openssh-client
RUN adduser -D runner
USER runner
WORKDIR /home/runner
COPY --from=build /app/target/release/ssh-deploy /usr/local/bin/ssh-deploy
ENTRYPOINT [ "ssh-deploy" ]
