#!/bin/bash

case "$1" in
"build")
	wasm-pack build --no-pack --target web --out-dir ./www/pkg
	rm ./www/pkg/.gitignore
	;;
"start")
	miniserve www --index "index.html" -p 8080
	;;
esac
