# SPDX-License-Identifier: GPL-2.0
.PHONY: all sections run r time clean
CFLAGS := -Wall -Werror -O2 -g
TARGET := sec331

all sections: $(TARGET)
$(TARGET): main.o lib.o
	$(CC) $(CFLAGS) $^ -o $@
%.o: %.c
	$(CC) $(CFLAGS) -c $<
run r: $(TARGET)
	./$<
lean:
	@$(RM) -rf $(TARGET)
