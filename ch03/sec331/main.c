/* 3.3.1 Spinlock by TTAS */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <assert.h>
#include <stdbool.h>
#include <pthread.h>

/* 32k threads seem to be the limit on my 8GB MBA7,1 */
#define NR_THREADS 32000

extern void spinlock_aquire(bool *lock);
extern void spinlock_release(bool *lock);

static bool lock = false;
static uint64_t counter = 0;

static void *worker(void *arg)
{
	intptr_t id = (intptr_t)arg;

	spinlock_aquire(&lock);
	counter++;
	spinlock_release(&lock);

	return (void *)id;
}

int main(int argc, const char *const argv[])
{
	const char *progname = argv[0];
	pthread_t *workers, *p;
	intptr_t i;

	workers = malloc(sizeof(pthread_t) * NR_THREADS);
	if (!workers)
		goto err;

	for (i = 0, p = workers; i < NR_THREADS; i++, p++) {
		int ret = pthread_create(p, NULL, worker, (void *)i);
		if (ret != 0)
			goto err;
	}
	for (i = 0, p = workers; i < NR_THREADS; i++, p++) {
		intptr_t id;
		int ret = pthread_join(*p, (void *)&id);
		if (ret != 0)
			goto err;
	}
	free(workers);

	printf("%s: counter = %ld\n", progname, counter);
	assert(counter == NR_THREADS);
	exit(EXIT_SUCCESS);
err:
	if (workers)
		free(workers);
	perror(progname);
	exit(EXIT_FAILURE);
}
