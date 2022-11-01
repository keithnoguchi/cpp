/* 4.4.0 Recursive Lock */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <pthread.h>
#include <assert.h>

#include "lib.h"

#define NR_WORKERS 10

static void *worker(void *arg)
{
	intptr_t id = (intptr_t)arg;
	char name[BUFSIZ];

	snprintf(name, sizeof(name), "worker%ld", id);
	printf("%s\n", name);

	return (void *)id;
}

int main(int argc, const char *const argv[])
{
	const char *progname = argv[0];
	pthread_t *workers, *p;
	int i;

	printf("%s: recursive lock with %d workers\n", progname, NR_WORKERS);

	workers = malloc(sizeof(pthread_t) * NR_WORKERS);
	if (workers == NULL)
		goto err;

	for (i = 0, p = workers; i < NR_WORKERS; i++, p++) {
		/* id == 0 is reserved for the init value */
		intptr_t id = i + 1;
		if (pthread_create(p, NULL, worker, (void *)id) != 0)
			goto err;
	}
	for (i = 0, p = workers; i < NR_WORKERS; i++, p++) {
		/* id == 0 is reserved for the init value */
		intptr_t id = i + 1;
		intptr_t got;
		if (pthread_join(*p, (void *)&got) != 0)
			goto err;
		assert(got == id);
	}
	exit(EXIT_SUCCESS);
err:
	perror(progname);
	if (workers)
		free(workers);
	exit(EXIT_FAILURE);
}
