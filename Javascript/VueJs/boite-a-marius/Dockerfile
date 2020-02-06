FROM debian

RUN apt-get update && apt-get upgrade -y

WORKDIR /boiteamarius
COPY Backend/target/release/BoiteAMarius .
RUN mkdir -p assets
COPY Front/dist assets

CMD ["./BoiteAMarius"]
