#!/bin/bash

case "$1" in
"build")
	wasm-pack build --out-dir ./doc/pkg
	;;
"start")
	npm --prefix doc run start
	;;
esac
