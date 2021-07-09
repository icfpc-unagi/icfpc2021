FROM golang:1.16.5 AS golang

FROM golang AS builder

RUN mkdir -p /work
WORKDIR /work
COPY go.mod go.sum /work/
RUN go mod download

COPY ./cmd /work/cmd
COPY ./pkg /work/pkg
COPY ./internal /work/internal
RUN go build -o /work/server ./cmd/server

FROM golang AS tini
RUN wget -O /tini \
        https://github.com/krallin/tini/releases/download/v0.18.0/tini \
    && chmod +x /tini

FROM golang
ARG UNAGI_PASSWORD
ARG UNAGI_PORTAL_PASSWORD
ARG UNAGI_API_KEY
COPY --from=builder /work/server /usr/local/bin/server
RUN [ "${UNAGI_PASSWORD}" != "" ]
RUN [ "${UNAGI_PORTAL_PASSWORD}" != "" ]
RUN [ "${UNAGI_API_KEY}" != "" ]
ENV SQL_ADDRESS 34.146.137.27
ENV SQL_USER root
ENV SQL_DATABASE database
ENV SQL_PASSWORD $UNAGI_PASSWORD
ENV UNAGI_API_KEY $UNAGI_API_KEY
ENV UNAGI_PORTAL_PASSWORD $UNAGI_PORTAL_PASSWORD
COPY --from=tini /tini /tini
ENTRYPOINT /tini -- /usr/local/bin/server --logtostderr
