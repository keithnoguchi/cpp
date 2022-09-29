# SPDX-License-Identifier: GPL-2.0
.PHONY: all sections run r clean
CFLAGS := -Wall -Werror -O2
# https://www.gnu.org/software/make/manual/make.html#toc-Quick-Reference-1
TARGET := $(notdir $(CURDIR))

all sections: $(TARGET)
$(TARGET): main.c
	$(CC) $(CFLAGS) $^ -o $@
run r: $(TARGET)
	@./$<
clean:
	@$(RM) -rf $(TARGET) *.o
