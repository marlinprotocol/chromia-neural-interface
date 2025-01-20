#!/bin/bash

# remove old images
sudo docker kill $(sudo docker ps -a -q)
sudo docker rm -f $(sudo docker ps -a -q)
sudo docker rmi -f $(sudo docker images -a -q)
sudo docker system prune -f

# build using the docker-compose file
# sudo docker-compose up --build
