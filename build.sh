#!/bin/bash

# remove old images
docker kill $(docker ps -a -q)
docker rm -f $(docker ps -a -q)
docker rmi -f $(docker images -a -q)
docker system prune -f

# load docker images
#docker load < ./ui/chromia-frontend.tar
docker load < ./chromia-node.tar

# build using the docker-compose file
docker-compose up -d 

sleep 10
# docker exec -it chromia-neural-interface_chr-node_1 sh -c 'bun run dev'
bun run dev
