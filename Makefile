.PHONY: all clean help doc

all:  linter	# build binary
	cargo build --release

clean:		# clean-up environment
	cargo clean

help:		# show this message
	@printf "Usage: make [OPTION]\n"
	@printf "\n"
	@perl -nle 'print $$& if m{^[\w-]+:.*?#.*$$}' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?#"} {printf "    %-18s %s\n", $$1, $$2}'

doc:		# show the document
	cargo doc --open

linter:	# run static analysis
	cargo check
	cargo fmt

test:	# run the test
	cargo test
	cargo bench
