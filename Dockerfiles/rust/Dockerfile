FROM rust:1.67-alpine

COPY Dockerfiles/killer.sh $HOME
WORKDIR $HOME

CMD rustc solution.rs; sh killer.sh ./solution