# Rough outline + TODO
- Blue version is docker container listening on port 8080
- if custom healtchecks fail, docker compose is restarted and the new environment brought up
- Forward all requests on port 8080 to a specific docker container listening on PORT 8085 (Blue)
- make requests to a custom healthcheck for blue
- if it fails, redirect all requests to 8084 (Green)

# Docker network notes
Bridge network connects network on host machine to the containers themselves

use `docker inspect <container-name>` to get information about the containers network

`docker network create <bridge-name>` creates a new network bridge on the host

To view the host ip for a specific network, use `docker network ls` to view the details of all bridges, get the network id then `ip a | grep <network-id>`

If two containers are on the same bridge you can ping them by using the name of the container

Using macVlans you can set different IP addresses for each container

