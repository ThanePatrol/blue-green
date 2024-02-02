#!/usr/bin/env bash

docker rm $(docker ps -a -q) -f

cd dummy-service
docker-compose up -d --build
#COLOR="green" docker buildx build -t green:latest . && docker run -d green:latest

#COLOR="blue" docker buildx build -t blue:latest . && docker run -d blue:latest
