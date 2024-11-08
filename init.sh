#!/bin/bash

cat Cargo.toml | sed "s/app-name/${APP_NAME}/g" > temp && rm -rf Cargo.toml && mv temp Cargo.toml

cargo update
