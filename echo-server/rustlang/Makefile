.PHONY: help
help:
	unmake $(MAKEFILE_LIST)


.PHONY: simple
simple:
	cd simple && cargo build && target/debug/simple --host 127.0.0.1 --port 9999
