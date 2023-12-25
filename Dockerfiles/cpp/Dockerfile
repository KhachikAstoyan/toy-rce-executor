FROM alpine:latest

COPY Dockerfiles/killer.sh $HOME
WORKDIR $HOME
RUN apk add --no-cache g++ musl-dev
CMD g++ -o solution solution.cpp; sh killer.sh ./solution