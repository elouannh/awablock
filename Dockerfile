FROM rust:1.80

COPY ./ ./

CMD ["./scripts/launch.sh"]
