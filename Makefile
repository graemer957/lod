# Idea came from https://github.com/cloudflare/securitytxt-worker/blob/master/Makefile
#
# See https://makefiletutorial.com/

unit:
	cargo test

coverage:
	@./ccov.sh apoc

test: unit coverage

cloc:
	cloc . --include-lang=Rust,Markdown,TOML,make,"Bourne Shell"

.PHONY: unit coverage test cloc

# Last updated: 20231111
