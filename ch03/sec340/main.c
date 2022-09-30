/* 3.4.0 Semaphore */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <pthread.h>
#include <assert.h>

/* in lib.c */
extern void semaphore_acquire(const int max_sem, volatile int *sem);
extern void semaphore_release(volatile int *sem);

#define NR_THREADS 20000
#define MAX_SEM    1 /* 1 makes the semaphore behaves as a mutex */

static int sem = 0;
static uint64_t counter = 0;

static void *worker(void *arg)
{
	intptr_t id = (intptr_t)arg;

	semaphore_acquire(MAX_SEM, &sem);
	counter++;
	semaphore_release(&sem);

	return (void *)id;
}

int main(int argc, const char *const argv[])
{
	const char *progname = argv[0];
	pthread_t *workers, *p;
	intptr_t id;

	workers = malloc(sizeof(pthread_t) * NR_THREADS);
	if (!workers)
		goto err;

	for (p = workers, id = 0; id < NR_THREADS; p++, id++) {
		int ret = pthread_create(p, NULL, worker, (void *)id);
		if (ret != 0)
			goto err;
	}
	for (p = workers, id = 0; id < NR_THREADS; p++, id++) {
		intptr_t got;
		int ret = pthread_join(*p, (void *)&got);
		if (ret != 0)
			goto err;
	}
	printf("%s: counter=%ld\n", progname, counter);
	assert(counter == NR_THREADS);
	free(workers);
	exit(EXIT_SUCCESS);
err:
	perror(progname);
	if (workers)
		free(workers);
	exit(EXIT_FAILURE);
}
