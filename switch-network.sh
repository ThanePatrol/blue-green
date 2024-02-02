#!/usr/bin/env bash

#invoke with the port parameter

sudo iptables -t nat -A PREROUTING -p tcp --dport 8000 -j REDIRECT --to-port $1
