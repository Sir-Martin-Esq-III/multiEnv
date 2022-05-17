SHELL := /bin/bash

build:
	cargo build --release
	mkdir ~/.multienv	
	cp -r ./target/release/ ~/.multienv/
	cp ./multiEnvrc.json ~/.multienv/