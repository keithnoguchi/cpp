# SPDX-License-Identifier: GPL-2.0
.PHONY: all sections r run c clean
CFLAGS := -Wall -Werror -O2
TARGET := $(notdir $(CURDIR))

all sections: $(TARGET)
$(TARGET): main.o lib.o
	$(CC) $(CFLAGS) $^ -o $@
%.o: %.c
	@$(CC) $(CFLAGS) -c $<
r run: $(TARGET)
	@./$<
c clean:
	@$(RM) -f $(TARGET) *.o
