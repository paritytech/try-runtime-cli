FROM rust:1.73-bullseye

RUN set -x

RUN apt-get update \
    && apt-get -y upgrade

COPY install.sh .
RUN ./install.sh

RUN set +x

ENV CARGO_TERM_COLOR=always

CMD bash
