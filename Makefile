.PHONY: all clean help doc

BIN=target/release/req

all: $(BIN)	# build binary

clean:		# clean-up environment
	cargo clean

help:		# show this message
	@printf "Usage: make [OPTION]\n"
	@printf "\n"
	@perl -nle 'print $$& if m{^[\w-]+:.*?#.*$$}' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?#"} {printf "    %-18s %s\n", $$1, $$2}'

doc:		# show the document
	cargo doc --open

$(BIN): $(wildcard src/*.rs) test
	cargo build --release

test:
	@cargo check
	@cargo fmt
	@cargo test
	@cargo bench
