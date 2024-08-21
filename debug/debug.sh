#!/bin/bash

declare -a addresses=("http://localhost:8080" "http://localhost:4040")

for addr in "${addresses[@]}"
do
	declare -a routes=(" " "user")
	echo -e "\tTesting routes for $addr"
	count=0
	for route in "${routes[@]}"
	do
		echo -e "\n$count:\t$addr/$route"
		curl $addr/$route
		count+=1
	done
	echo -e "\n"
done
