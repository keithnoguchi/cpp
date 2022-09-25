# SPDX-License-Identifier: GPL-2.0
.PHONY: all
CFLAGS := -O3 -S

all sections: compare_and_swap_not.c compare_and_swap.c
	@$(CC) $(CFLAGS) $^
clean:
	@$(RM) *.s
