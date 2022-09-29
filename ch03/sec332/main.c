/* 3.3.2 Posix Mutex */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <pthread.h>
#include <assert.h>

#define NR_THREADS 20000
#define WORKER_PREFIX "worker"

static pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
static int64_t counter = 0;

static void *worker(void *arg)
{
	intptr_t id = (intptr_t)arg;
	char name[BUFSIZ]; /* e.g., worker2000 */
	int ret;

	ret = pthread_mutex_lock(&lock);
	if (ret != 0)
		goto err;
	counter++;
	ret = pthread_mutex_unlock(&lock);
	if (ret != 0)
		goto err;
	return (void *)id;
err:
	snprintf(name, sizeof(name) - 1, "%s%ld", WORKER_PREFIX, id);
	perror(name);
	return (void *)id;;
}

int main(int argc, const char *const argv[])
{
	const char *progname = argv[0];
	pthread_t *workers, *p;
	intptr_t id;
	int ret;

	workers = malloc(sizeof(pthread_t) * NR_THREADS);
	if (!workers)
		goto err;

	for (id = 0, p = workers; id < NR_THREADS; id++, p++) {
		ret = pthread_create(p, NULL, worker, (void *)id);
		if (ret != 0)
			goto err;
	}
	for (id = 0, p = workers; id < NR_THREADS; id++, p++) {
		intptr_t got;
		ret = pthread_join(*p, (void **)&got);
		if (ret != 0) {
			fprintf(stderr, "worker%ld: join error", id);
			goto err;
		}
	}
	printf("counter = %ld\n", counter);
	assert(counter == NR_THREADS);
	free(workers);
	ret = pthread_mutex_destroy(&lock);
	if (ret != 0)
		perror(progname);
	exit(EXIT_SUCCESS);
err:
	perror(progname);
	if (workers)
		free(workers);
	pthread_mutex_destroy(&lock);
	exit(EXIT_FAILURE);
}
