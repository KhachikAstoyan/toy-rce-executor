#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: $0 <command>"
    exit 1
fi

"$@" &

pid=$!

(sleep 20 && kill $pid) &

wait $pid

if [ $? != 0 ]; then
    kill $pid
fi
