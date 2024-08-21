#!/bin/bash

ENV_FILE=".env"

if [ -f "$ENV_FILE" ]; then
	export $(grep -v '#^' $ENV_FILE | xargs)
else
	exit 1
fi

cargo build --release

if [ $? -ne 0 ]; then
    echo "La compilation a échoué."
    exit 1
fi

./target/release/awablock
