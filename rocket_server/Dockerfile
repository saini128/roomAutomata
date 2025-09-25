FROM debian:bookworm-slim

WORKDIR /app

COPY rocket_server /app/
COPY Rocket.toml /app/

EXPOSE 8001
RUN chmod +x /app/rocket_server
CMD ["./rocket_server"]
