/* SPDX-License-Identifier: GPL-2.0 */
#include <stdio.h>
#include <stdlib.h>

static int *f2(int a)
{
	int *tmp = malloc(sizeof(int));
	if (tmp == NULL)
		perror("malloc");
	*tmp = 2 * a;
	return tmp;
}

static void f1()
{
	int a = 10;
	int *b = f2(a);
	printf("a = %d, b = %d\n", a, *b);
	free(b);
}

int main(int argc, char *argv[])
{
	printf("sec223: ");
	f1();
	exit(EXIT_SUCCESS);
}
