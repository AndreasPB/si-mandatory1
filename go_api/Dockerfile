ARG APP_ENVIRONMENT=dev

FROM golang:alpine as base

WORKDIR /app

EXPOSE 9901

COPY go.mod go.sum ./

RUN go mod download


COPY *.go ./

RUN CGO_ENABLED=0 go build .

FROM base as dev
WORKDIR /app

RUN go install github.com/cosmtrek/air@latest
CMD air

FROM scratch as prod
WORKDIR /app

ENV GIN_MODE=release
COPY --from=base /app/go_api /usr/bin/
CMD ["go_api"]
