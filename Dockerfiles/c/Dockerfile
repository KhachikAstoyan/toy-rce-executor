FROM alpine:latest

COPY Dockerfiles/killer.sh $HOME
WORKDIR $HOME
RUN apk add --no-cache gcc musl-dev

CMD gcc -o solution solution.c; sh killer.sh ./solution