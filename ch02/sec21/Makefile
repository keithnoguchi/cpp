# SPDX-License-Identifier: GPL-2.0
PROG := sec221
CFLAGS := -Wall -Werror

all: $(PROG)
$(PROG): main.c
	$(CC) $(CFLAGS) -o $@ $<
.PHONY: clean
clean:
	@$(RM) $(PROG)
