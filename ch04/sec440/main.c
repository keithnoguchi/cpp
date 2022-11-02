/* 4.4.0 Recursive Lock */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <pthread.h>
#include <assert.h>

#include "lib.h"

#define NR_WORKERS 10000
#define NR_RECURSIVE 5

/* global recursive lock shared among multiple workers */
static struct recursive_lock LOCK;

/* global counter protected by the recursive lock */
static uint64_t counter = 0;

/* recursively call to demonstrate the recursive lock */
static void countup(intptr_t id, int limit)
{
	if (limit == 0)
		return;
	recursive_lock_acquire(&LOCK, id);
	counter++;
	countup(id, --limit);
	recursive_lock_release(&LOCK, id);
}

static void *worker(void *arg)
{
	intptr_t id = (intptr_t)arg;

	countup(id, NR_RECURSIVE);

	return (void *)id;
}

int main(int argc, const char *const argv[])
{
	const char *progname = argv[0];
	pthread_t *workers, *p;
	int i;

	printf("%s: %d recursive locks by %d workers\n",
	       progname, NR_RECURSIVE, NR_WORKERS);

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
	assert(counter == NR_WORKERS * NR_RECURSIVE);
	exit(EXIT_SUCCESS);
err:
	perror(progname);
	if (workers)
		free(workers);
	exit(EXIT_FAILURE);
}
