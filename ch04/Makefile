# SPDX-License-Identifier: GPL-2.0
.PHONY: all chapters

all chapters: sections
	@cargo check
	@cargo clippy
	@cargo build
%:
	@$(MAKE) -C sec440 $@
	@$(MAKE) -C sec450 $@
	@$(MAKE) -C sec460 $@
