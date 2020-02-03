DOCKER_IMAGE=ekidd/rust-musl-builder:1.41.0
NAME=$(notdir $(PWD))
BIN=target/debug/$(NAME)
DOCKER=docker run --rm -it -v $(PWD):/home/rust/src $(DOCKER_IMAGE)
COMPILE=cargo build

.PHONY: all compile console

all: compile run

run:
	@$(BIN)

compile:
	$(COMPILE)

release:
	$(DOCKER) $(COMPILE) --release

console:
	$(DOCKER) bash
