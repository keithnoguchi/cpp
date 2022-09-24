# SPDX-License-Identifier: GPL-2.0
.PHONY: all clean subsystem

all: clean subsystem
	cargo check
	cargo clippy
	cargo build

# https://www.gnu.org/software/make/manual/html_node/Recursion.html
subsystem:
	$(MAKE) -C ch02
	$(MAKE) -C ch03/sec42

clean:
	cargo clean
	$(MAKE) -C ch02 $@
	$(MAKE) -C ch03/sec42 $@
