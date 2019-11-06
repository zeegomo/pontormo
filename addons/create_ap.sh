#!/bin/bash

if [[ $# != 1 ]]; then
	echo "invalid arguments"
	echo "need name of the wireless interface"
	exit
fi
create_ap -n $1 pontormo --isolate-clients &
docker build -t pontormo .
docker run -p 80:80 -it pontormo 
