# SPDX-License-Identifier: GPL-2.0
.PHONY: all sections clean
CFLAGS := -Wall -Werror
TARGET := sec342

all sections: $(PROG)
$(PROG): main.c
	@$(CC) $(CFLAGS) $^ -o $(TARGET)
run r: $(TARGET)
	@./$<
clean:
	@$(RM) $(PROG)
