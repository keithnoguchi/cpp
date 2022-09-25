# SPDX-License-Identifier: GPL-2.0
.PHONY: all chapters

all chapters: clean sections
	cargo check
	cargo clippy
	cargo build
%:
	@$(MAKE) -C sec342 $@
