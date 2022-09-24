# SPDX-License-Identifier: GPL-2.0
.PHONY: all clean sections

all: clean sections
	cargo check
	cargo clippy
	cargo build

# https://www.gnu.org/software/make/manual/html_node/Recursion.html
sections:
	$(MAKE) -C sec221
	$(MAKE) -C sec222
	$(MAKE) -C sec223

clean:
	cargo clean
	$(MAKE) -C sec221 $@
	$(MAKE) -C sec222 $@
	$(MAKE) -C sec223 $@

