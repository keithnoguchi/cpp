# SPDX-License-Identifier: GPL-2.0
.PHONY: all sections r run c clean
CFLAGS := -Wall -Werror
TARGET := $(notdir $(CURDIR))

all sections: $(TARGET)
$(TARGET): main.c
	@$(CC) $(CFLAGS) $^ -o $@
r run: $(TARGET)
	@./$<
c clean:
	@$(RM) -f $(TARGET) *.o
