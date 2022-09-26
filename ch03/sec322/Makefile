# SPDX-Liencese-Identifier: GPL-2.0
.PHONY: all
CFLAGS := -O3 -S -Wimplicit-function-declaration

all sections: test_and_set_not.c test_and_set.c
	@$(CC) $(CFLAGS) $^
clean:
	@$(RM) *.s
