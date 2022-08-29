# SPDX-License-Identifier: GPL-2.0
PROG := sec342
CFLAGS := -Wall -Werror

all: $(PROG)
$(PROG): main.c
	$(CC) $(CFLAGS) -o $(PROG) $<
.PHONY: clean
clean:
	@$(RM) $(PROG)
