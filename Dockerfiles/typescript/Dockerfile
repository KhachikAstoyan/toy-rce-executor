FROM node:20-alpine

COPY Dockerfiles/killer.sh $HOME
WORKDIR $HOME
RUN npm install -g typescript

CMD tsc solution.ts; sh killer.sh node solution.js