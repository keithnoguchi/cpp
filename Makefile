# SPDX-License-Identifier: GPL-2.0
.PHONY: all

all: chapters
	@cargo check
	@cargo clippy
	@cargo build
# https://www.gnu.org/software/make/manual/html_node/Recursion.html
%:
	@$(MAKE) -C ch02 $@
	@$(MAKE) -C ch03 $@
