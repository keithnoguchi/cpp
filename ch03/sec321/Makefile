# SPDX-License-Identifier: GPL-2.0
.PHONY: all sections r run c clean
CFLAGS := -O3 -S -Wimplicit-function-declaration

all sections: compare_and_swap_not.c compare_and_swap.c
	@$(CC) $(CFLAGS) $^
r run: all
c clean:
	@$(RM) -f *.s
