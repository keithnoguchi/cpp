# SPDX-License-Identifier: GPL-2.0
.PHONY: all sections r run c clean
CFLAGS := -Wall -Werror -O2 -g
LFLAGS := -pthread
TARGET := $(notdir $(CURDIR))

all sections: $(TARGET)
$(TARGET): main.o lib.o
	$(CC) $(LFLAGS) $^ -o $@
%.o: %.c
	$(CC) $(CFLAGS) -c $<
r run: $(TARGET)
	./$<
c clean:
	@$(RM) -rf $(TARGET) *.o
