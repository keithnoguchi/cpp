/* 3.6.1 Memory Barrier by Spinlock */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <pthread.h>
#include <assert.h>

/* 200 is a reasonal number I can go without melting my 8GiB MBA. ;) */
#define NR_THREADS 200

extern void barrier(volatile int *cnt, const int max);
static volatile int counter = 0;

static void *worker(void *arg)
{
	intptr_t id = (intptr_t)arg;
	char name[BUFSIZ];

	/* wait on the barrier */
	snprintf(name, sizeof(name), "worker%ld", id);
	barrier(&counter, NR_THREADS);

	return (void *)id;
}

int main(int argc, const char *const argv[])
{
	const char *progname = argv[0];
	pthread_t *p, *workers;
	intptr_t id;

	workers = malloc(sizeof(pthread_t) * NR_THREADS);
	if (!workers)
		goto err;

	for (p = workers, id = 0; id < NR_THREADS; p++, id++)
		if (pthread_create(p, NULL, worker, (void *)id) != 0)
			goto err;

	for (p = workers, id = 0; id < NR_THREADS; p++, id++) {
		intptr_t ret;
		if (pthread_join(*p, (void **)&ret) != 0)
			goto err;
		if (ret != id)
			goto err;
	}

	free(workers);
	printf("%s: counter=%d\n", progname, counter);
	assert(counter == NR_THREADS);

	exit(EXIT_SUCCESS);
err:
	perror(progname);
	if (workers)
		free(workers);

	exit(EXIT_FAILURE);
}
