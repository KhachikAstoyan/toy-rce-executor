FROM alpine:latest

COPY Dockerfiles/killer.sh $HOME
WORKDIR $HOME
RUN apk add --no-cache bash

CMD sh killer.sh bash solution.sh