#!/bin/bash

dockerFileDir="./Dockerfiles"

folders=$(ls "$dockerFileDir")
echo $folders

for folder in $folders; do
  languageDir="$dockerFileDir/$folder"


  if [ ! -d "$languageDir" ]; then
    continue
  fi

  dockerfilePath="$languageDir/Dockerfile"

  if [ -e "$dockerfilePath" ]; then
    docker build -q -t "toyrce:$folder" -f "$dockerfilePath" . &
  fi
done

exit
