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
	@$(MAKE) -C sec331 $@
	@$(MAKE) -C sec332 $@
	@$(MAKE) -C sec340 $@
	@$(MAKE) -C sec342 $@
	@$(MAKE) -C sec342 $@
	@$(MAKE) -C sec350 $@
	@$(MAKE) -C sec361 $@
	@$(MAKE) -C sec362 $@
	@$(MAKE) -C sec371 $@
