#!/bin/bash

case "$1" in
"build")
	wasm-pack build --target bundler --out-dir ./www/pkg
	rm ./www/pkg/.gitignore
	;;
"build-all")
	wasm-pack build --target bundler --out-dir ./www/pkg
	rm ./www/pkg/.gitignore
	npm --prefix www run build
	;;
"serve")
	npm --prefix www run serve
	;;
"serve-dist")
	miniserve www/dist --index "index.html" -p 8080
	;;
esac
