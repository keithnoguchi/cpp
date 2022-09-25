# SPDX-License-Identifier: GPL-2.0
.PHONY: all sections clean run r
PROG := sec223
CFLAGS := -Wall -Werror

all sections: $(PROG) run
$(PROG): main.c
	$(CC) $(CFLAGS) -o $@ $^
run r:
	@./$(PROG)
clean:
	@$(RM) $(PROG)
