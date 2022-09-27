# SPDX-License-Identifier: GPL-2.0
.PHONY: all run r
CFLAGS := -Wall -O2
TARGET := sec330

all sections: $(TARGET)
$(TARGET): main.c
	@$(CC) $(CFLAGS) $^ -o $@
run r: $(TARGET)
	@./$<
clean:
	@$(RM) $(TARGET)
