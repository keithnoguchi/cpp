# SPDX-License-Identifier: GPL-2.0
.PHONY: all clean subsystem

all: clean subsystem
	cargo clippy
	cargo build

# https://www.gnu.org/software/make/manual/html_node/Recursion.html
subsystem:
	$(MAKE) -C ch03/sec342

clean:
	$(MAKE) -C ch03/sec342 $@
