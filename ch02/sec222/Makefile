# SPDX-License-Identifier: GPL-2.0
all: wait_while.c wait_while_volatile.c
	$(CC) -O3 -S $^
.PHONY: clean
clean:
	@$(RM) *.s

