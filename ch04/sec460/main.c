/* 4.6.0 Signal Handler in C */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <signal.h>
#include <pthread.h>
#include <unistd.h>
#include <assert.h>

/* in lib.c */
extern uint64_t worker_counter;
extern const uint64_t nr_worker_loop;
extern int init_signal_handlers(int, uint64_t nr);
extern void *worker(void *arg);

#define NR_WORKERS 10
#define NR_HANDLERS 5

int main(int argc, const char *const argv[])
{
	const char *progname = argv[0];
	pthread_t *p, *workers = NULL;
	pid_t pid = getpid();
	intptr_t id;

	printf("%s(pid=%d): %d workers with %d signal handlers\n",
	       progname, pid, NR_WORKERS, NR_HANDLERS);

	/*
	 * Initialize signals and the handlers.
	 *
	 * Call this before the worker creation
	 * to reflect the signal mask.
	 */
	if (init_signal_handlers(SIGUSR1, NR_HANDLERS) != 0)
		goto err;

	/* workers */
	workers = malloc(sizeof(pthread_t) * NR_WORKERS);
	if (!workers)
		goto err;

	for (id = 0, p = workers; id < NR_WORKERS; id++, p++)
		if (pthread_create(p, NULL, worker, (void *)id) != 0)
			goto err;

	for (id = 0, p = workers; id < NR_WORKERS; id++, p++) {
		intptr_t got;
		if (pthread_join(*p, (void *)&got) != 0)
			goto err;
		assert(got == id);
	}
	assert(worker_counter == NR_WORKERS * nr_worker_loop);
	if (workers)
		free(workers);

	exit(EXIT_SUCCESS);
err:
	if (workers)
		free(workers);
	perror(progname);
	exit(EXIT_FAILURE);
}
