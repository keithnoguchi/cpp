# SPDX-License-Identifier: GPL-2.0
.PHONY: all sections r run c clean

all sections: wait_while.c wait_while_volatile.c
	$(CC) -O3 -S $^
r run: ;
c clean:
	@$(RM) -f *.s
