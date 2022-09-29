# SPDX-Liencese-Identifier: GPL-2.0
.PHONY: all sections r run c clean
CFLAGS := -O3 -S -Wimplicit-function-declaration

all sections: test_and_set_not.c test_and_set.c
	@$(CC) $(CFLAGS) $^
# https://www.gnu.org/s/make/manual/html_node/Empty-Recipes.html
r run: ;
c clean:
	@$(RM) -f *.s
