# SPDX-License-Identifier: GPL-2.0
.PHONY: all chapters

all chapters: sections
	@cargo check
	@cargo clippy
	@cargo build
%:
	$(MAKE) -C sec221 $@
	$(MAKE) -C sec222 $@
	$(MAKE) -C sec223 $@
