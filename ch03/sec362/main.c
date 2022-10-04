/* 3.6.2 Memory Barrier by Posix Condition Variables */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <pthread.h>

/*
 * 20k workers seems to be the memroy limit on my 8G MBA.
 *
 * It's 100 times more workers than the [spinlock] version, though.
 *
 * [spinlock]: * https://github.com/keithnoguchi/cpr/ch03/sec361/main.c
 */
#define NR_THREADS 20000

/* in lib.c */
extern void barrier(volatile int *cnt, const int max, const char *name);

static volatile int counter = 0;

static void *worker(void *arg)
{
	intptr_t id = (intptr_t)arg;
	char name[BUFSIZ];

	snprintf(name, sizeof(name), "worker%ld", id);
	barrier(&counter, NR_THREADS, name);

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
		int ret;
		if (pthread_join(*p, (void **)&ret) != 0)
			goto err;
		if (ret != id) {
			fprintf(stderr, "worker%ld: error\n", id);
			goto err;
		}
	}

	printf("%s\n", progname);
	free(workers);

	exit(EXIT_SUCCESS);
err:
	perror(progname);
	if (workers)
		free(workers);

	exit(EXIT_FAILURE);
}
