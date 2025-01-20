#!/bin/bash

# remove old images
docker kill $(docker ps -a -q)
docker rm -f $(docker ps -a -q)
docker rmi -f $(docker images -a -q)
docker system prune -f

# build using the docker-compose file
docker-compose up --build
