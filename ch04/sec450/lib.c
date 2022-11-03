/* 4.5.0 Spurious Wakeup */
#include <stdio.h>
#include <stdint.h>
#include <unistd.h>
#include <pthread.h>
#include <time.h>
#include <errno.h>

#define TIMEOUT_MS 10000 /* 10s */

static pthread_cond_t cond = PTHREAD_COND_INITIALIZER;
static pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;

void *worker(void *arg)
{
	struct timespec timeout = {
		.tv_sec = TIMEOUT_MS / 1000,
		.tv_nsec = (TIMEOUT_MS % 1000) * 1000,
	};
	intptr_t id = (intptr_t)arg;
	pid_t pid = getpid();
	char name[BUFSIZ];
	time_t now;
	int ret;

	snprintf(name, sizeof(name) - 1, "worker%ld(pid=%d)", id, pid);
	printf("%s\n", name);

	now = time(NULL);
	if (now == -1)
		goto err;

	if (pthread_mutex_lock(&lock) != 0)
		goto err;

	/* timed wait to avoid blocking forever */
	timeout.tv_sec += now; /* absolute time */
	ret = pthread_cond_timedwait(&cond, &lock, &timeout);
	if (ret != ETIMEDOUT)
		goto err;

	if (pthread_mutex_unlock(&lock) != 0)
		goto err;

	return (void *)id;
err:
	perror(name);
	return (void *)-1;
}
