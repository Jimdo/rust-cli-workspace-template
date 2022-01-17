FROM debian:bullseye-slim

ENV DEBIAN_FRONTEND=noninteractive

COPY ./target/release/${{values.name | dump}} /usr/local/bin/${{values.name | dump}}

CMD ["${{values.name | dump}}"]
