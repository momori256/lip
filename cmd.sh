#!/bin/bash

case "$1" in
"build")
	wasm-pack build --no-pack --out-dir ./www/pkg
	rm ./www/pkg/.gitignore ./www/pkg/README.md
	;;
"start")
	npm --prefix www install
	npm --prefix www run start
	;;
esac
