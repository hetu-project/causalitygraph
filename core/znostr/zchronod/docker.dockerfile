
FROM ubuntu

WORKDIR /usr/chronod
RUN mkdir -p /usr/chronod/config
RUN mkdir -p /usr/chronod/log
RUN mkdir -p /usr/chronod/bin

COPY config.yaml /usr/chronod/config/config.yaml

RUN cargo build --release -o zchronod

COPY  zchronod /usr/chronod/bin/zchronod

CMD ["zchronod","-C","/usr/chronod/config.yaml"]