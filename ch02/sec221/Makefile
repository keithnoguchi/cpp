# SPDX-License-Identifier: GPL-2.0
.PHONY: all sections clean
PROG := sec221
CFLAGS := -Wall -Werror

all sections: $(PROG)
$(PROG): main.c
	@$(CC) $(CFLAGS) -o $@ $^
clean:
	@$(RM) $(PROG)
