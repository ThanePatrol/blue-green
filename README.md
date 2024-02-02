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

# TODO for demo
1. Two instances of SP Ingress running locally, One blue (8084) one green (8085)
2. Basic rust service Dockerized that runs a healthcheck every second
3. When Blue fails healthcheck, we fall back to green - Rust container issues a command
4. Need to figure out command

# QEMU setup
Download latest ubuntu arm version https://ubuntu.com/download/server

`qemu-img create -f qcow2 ubuntu_arm.qcow2 20G`
```shell
   qemu-system-arm -M virt -m 1024 \
   -cpu cortex-a15 \
   -bios /usr/local/share/qemu/edk2-aarch64-code.fd \
   -drive if=none,file=ubuntu_arm.qcow2,format=qcow2,id=hd \
   -device virtio-blk-device,drive \
   -kernel ~/Downloads/ubuntu_arm.qcow2

```
