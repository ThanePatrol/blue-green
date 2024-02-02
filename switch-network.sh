#!/usr/bin/env bash

#invoke with the port parameter
#must been run as root on a linux machine
#
# This should be run at least once sysctl -w net.ipv4.ip_forward=1
#
# Need OUTPUT for local machine
iptables -t nat -D OUTPUT -p tcp --dport 8000 -j REDIRECT --to-port $1
iptables -t nat -A OUTPUT -p tcp --dport 8000 -j REDIRECT --to-port $2 

# iptables -t nat -A PREROUTING -p tcp --dport 8000 -j REDIRECT --to-port $1 won't work on local
