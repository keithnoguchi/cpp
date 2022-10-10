# SPDX-License-Identifier: GPL-2.0
.PHONY: all sections r run b bench c clean
CFLAGS := -Wall -Werror -O2
LFLAGS := -pthread
TARGET := $(notdir $(CURDIR))

all sections: $(TARGET)
$(TARGET): main.o lib.o
	@$(CC) $(LFLAGS) $^ -o $@
%.o: %.c
	@$(CC) $(CFLAGS) -c $<
r run b bench: $(TARGET)
	@./$<
c clean:
	@$(RM) -f $(TARGET) *.o
# Selecting the different lock mechanism
bench-%: $(TARGET)
	@$(info "$(patsubst bench-%,%,$@) benchmark")
	@./$< $(patsubst bench-%,%,$@)
