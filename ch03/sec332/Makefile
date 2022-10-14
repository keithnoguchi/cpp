# SPDX-License-Identifier: GPL-2.0
.PHONY: all sections r run c clean
CFLAGS := -Wall -Werror -O2
LFLAGS := -pthread
TARGET := $(notdir $(CURDIR))

all sections: $(TARGET)
$(TARGET): main.c
	$(CC) $(LFLAGS) $^ -o $@
r run: $(TARGET)
	@./$<
c clean:
	@$(RM) -f $(TARGET) *.o
