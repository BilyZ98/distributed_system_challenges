# syntax=docker/dockerfile:1
# FROM golang:1.16-alpine AS build
# FROM eclipse-temurin:11 AS build
FROM ubuntu AS base
RUN apt-get update && \
    apt-get install -y openjdk-11-jdk && \
    apt-get install -y gnuplot && \
    apt-get install -y graphviz && \
    apt-get install -y curl  && \
    apt-get install -y wget && \
    apt-get install -y bzip2 && \ 
    apt-get install -y git   && \
    apt-get install -y gcc 

# RUN apt-get install -y git 

# WORKDIR /go/src/github.com/org/repo


# COPY . .
# RUN go build -o server .
RUN wget https://github.com/jepsen-io/maelstrom/releases/download/v0.2.3/maelstrom.tar.bz2 && \
    tar -xvf maelstrom.tar.bz2  

RUN wget https://go.dev/dl/go1.20.4.linux-amd64.tar.gz &&  \
    rm -rf /usr/local/go && \
    tar -C /usr/local -xzf go1.20.4.linux-amd64.tar.gz 

RUN curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf --output /tmp/rsinstall.sh \
    && chmod +x /tmp/rsinstall.sh && /tmp/rsinstall.sh -y

ENV PATH=$PATH:/usr/local/go/bin 


# RUN go get github.com/jepsen-io/maelstrom/demo/go && \
    # go install . 

# FROM build AS development


# CMD ["/bin/bash"]

CMD cd /usr/src/project/maelstrom-echo && go get github.com/jepsen-io/maelstrom/demo/go && go install  -buildvcs=false .  && bash
# RUN apk update \
#     && apk add git
# CMD ["go", "run", "main.go"]
# FROM alpine:3.12
# EXPOSE 8000
# COPY --from=build /go/src/github.com/org/repo/server /server
# CMD ["/server"]
