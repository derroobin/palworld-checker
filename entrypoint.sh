#!/bin/sh

printenv | grep -v "no_proxy" >> /etc/environment

# execute CMD
echo "$@"
exec "$@"