# SPDX-License-Identifier: GPL-2.0
.PHONY: all chapters

all chapters: sections
	@cargo check
	@cargo clippy
	@cargo build
%:
	@$(MAKE) -C sec321 $@
	@$(MAKE) -C sec322 $@
	@$(MAKE) -C sec330 $@
	@$(MAKE) -C sec342 $@
