#!/bin/bash

cat Cargo.toml | sed "s/template/${NAME}/g" > temp && rm -rf Cargo.toml && mv temp Cargo.toml

cargo update
