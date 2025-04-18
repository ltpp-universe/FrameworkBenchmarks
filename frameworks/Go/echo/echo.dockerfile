FROM docker.io/golang:1.24.2

ADD ./src /echo
WORKDIR /echo

RUN GOAMD64=v3 go build -ldflags="-s -w" -o app ./main.go

EXPOSE 8080

CMD ./app
