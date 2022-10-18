/* 3.3.0 Mutex by TAS */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <pthread.h>
#include <assert.h>

/* 20k is the maximum I can go on my 8GB MBA. :) */
#define NR_THREADS 20000

/* global lock */
static bool lock = false;
static uint64_t counter = 0;

static bool test_and_set(volatile bool *v)
{
	return __sync_lock_test_and_set(v, true);
}

static void tas_release(volatile bool *v)
{
	__sync_lock_release(v);
}

static void *worker(void *arg)
{
	intptr_t id = (intptr_t)arg;
retry:
	if (!test_and_set(&lock)) {
		//printf("worker%ld: in the critical section\n", id);
		counter++;
	} else
		goto retry;
	tas_release(&lock);
	return (void *)id;
}

int main(int argc, char *argv[])
{
	const char *progname = argv[0];
	pthread_t workers[NR_THREADS];
	intptr_t i;
	int ret;

	for (i = 0; i < NR_THREADS; i++) {
		ret = pthread_create(&workers[i], NULL, worker, (void *)i);
		if (ret != 0)
			goto err;
	}
	for (i = 0; i < NR_THREADS; i++) {
		intptr_t got;
		ret = pthread_join(workers[i], (void **)&got);
		if (ret != 0)
			goto err;
		if (got != i) {
			fprintf(stderr, "worker%d: error(%ld)\n", i, got);
			goto err;
		}

	}

	printf("counter = %ld\n", counter);
	assert(counter == NR_THREADS);
	return EXIT_SUCCESS;
err:
	perror(progname);
	return EXIT_FAILURE;
}
