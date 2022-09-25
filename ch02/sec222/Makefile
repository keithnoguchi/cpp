# SPDX-License-Identifier: GPL-2.0
.PHONY: all sections clean

all sections: wait_while.c wait_while_volatile.c
	$(CC) -O3 -S $^
clean:
	@$(RM) *.s
