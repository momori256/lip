#!/bin/bash

case "$1" in
"build")
	wasm-pack build
	;;
"start")
	npm --prefix www run start
	;;
esac
