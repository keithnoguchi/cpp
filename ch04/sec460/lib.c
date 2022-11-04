/* 4.6.0 Signal Handler in C */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <signal.h>
#include <pthread.h>
#include <errno.h>
#include <time.h>

#define NR_HANDLER_LOOP 5
#define NR_WORKER_LOOP 100
#define NR_WORKER_TIMEOUT_MS 10

/* counter incremented by the workers */
uint64_t worker_counter = 0;
const uint64_t nr_worker_loop = NR_WORKER_LOOP;

static pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
static sigset_t sigset;

static void *handler(void *arg)
{
	intptr_t id = (intptr_t)arg;
	char name[BUFSIZ];
	int got;

	snprintf(name, sizeof(name) - 1, "handler%ld", id);

	/* wait for the signal */
	for (int i = 0; i < NR_HANDLER_LOOP; i++) {
		if (sigwait(&sigset, &got) != 0)
			goto err;
		if (pthread_mutex_lock(&lock) != 0)
			goto err;
		printf("%s got signal(%d)\n", name, got);
		if (pthread_mutex_unlock(&lock) != 0)
			goto err;
	}

	return (void *)id;
err:
	perror(name);
	return (void *)-1;
}

int init_signal_handlers(int signal, uint64_t nr_handlers)
{
	pthread_t *p, *handlers = NULL;
	pthread_attr_t attr;
	intptr_t id;
	int ret;

	/* let's block the signal for all the threads. */
	if (sigemptyset(&sigset) != 0)
		goto err;
	if (sigaddset(&sigset, signal) != 0)
		goto err;
	if (pthread_sigmask(SIG_BLOCK, &sigset, NULL) != 0)
		goto err;

	handlers = malloc(sizeof(pthread_t) * nr_handlers);
	if (!handlers)
		goto err;

	memset(&attr, 0, sizeof(attr));
	ret = pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);
	if (ret != 0)
		goto err;

	for (id = 0, p = handlers; id < nr_handlers; id++, p++)
		if (pthread_create(p, &attr, handler, (void *)id) != 0)
			goto err;

	/* no tracking as the handlers will terminate on it's own */
	free(handlers);
	return 0;
err:
	if (handlers)
		free(handlers);
	return -errno;
}

void *worker(void *arg)
{
	static struct timespec timeout = {
		.tv_sec = NR_WORKER_TIMEOUT_MS / 1000,
		.tv_nsec = (NR_WORKER_TIMEOUT_MS % 1000) * 1000000,
	};
	intptr_t id = (intptr_t)arg;
	char name[BUFSIZ];

	snprintf(name, sizeof(name) - 1, "worker%ld", id);
	printf("%s fired\n", name);

	for (int i = 0; i < NR_WORKER_LOOP; i++) {
		if (pthread_mutex_lock(&lock) != 0)
			goto err;
		/* some work here... */
		worker_counter += 1;
		nanosleep(&timeout, NULL);
		if (pthread_mutex_unlock(&lock) != 0)
			goto err;
	}

	return (void *)id;
err:
	perror(name);
	return (void *)-1;
}
