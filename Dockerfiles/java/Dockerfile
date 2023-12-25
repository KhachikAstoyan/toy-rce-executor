FROM eclipse-temurin:11

COPY Dockerfiles/killer.sh $HOME
WORKDIR $HOME

CMD javac Main.java; sh killer.sh java Main