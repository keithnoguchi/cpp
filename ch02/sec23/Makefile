# SPDX-License-Identifier: GPL-2.0
PROG := sec223
CFLAGS := -Wall -Werror

all: $(PROG)
$(PROG): main.c
	$(CC) $(CFLAGS) -o $@ $^
.PHONY: run r clean
run r:
	@./$(PROG)
clean:
	@$(RM) $(PROG)
