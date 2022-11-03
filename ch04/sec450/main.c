/* 4.5.0 Spurious Wakeup */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <signal.h>
#include <unistd.h>
#include <pthread.h>
#include <assert.h>

/* from lib.c */
extern void *worker(void *arg);

#define NR_WORKERS 5

static void handler(int sig, siginfo_t *info, void *ctx)
{
	printf("signal %d\n", sig);
}

int main(int argc, const char *const argv[])
{
	const char *progname = argv[0];
	pthread_t *workers, *p;
	struct sigaction sa;
	intptr_t id;
	pid_t pid;

	pid = getpid();
	printf("%s: pid=%d with %d workers\n", progname, pid, NR_WORKERS);

	workers = malloc(sizeof(pthread_t) * NR_WORKERS);
	if (!workers)
		goto err;

	memset(&sa, 0, sizeof(sa));
	sa.sa_sigaction = handler;
	if (sigaction(SIGUSR1, &sa, NULL) != 0)
		goto err;

	for (id = 0, p = workers; id < NR_WORKERS; id++, p++)
		if (pthread_create(p, NULL, worker, (void *)id) != 0)
			goto err;

	for (id = 0, p = workers; id < NR_WORKERS; id++, p++) {
		intptr_t ret;
		if (pthread_join(*p, (void *)&ret) != 0)
			goto err;
		assert(ret == id);
	}
	free(workers);

	exit(EXIT_SUCCESS);
err:
	perror(progname);
	if (workers)
		free(workers);
	exit(EXIT_FAILURE);
}
