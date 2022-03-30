#!/bin/bash

cat Cargo.toml | sed "s/template/${NAME}/g" > temp && rm -rf Cargo.toml && mv temp Cargo.toml

cat Cargo.toml | sed "s/_description_/${DESC}/g" > temp && rm -rf Cargo.toml && mv temp Cargo.toml

cargo update


