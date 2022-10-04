/* 3.6.2 Memory Barrier by Posix Condition Variable */
#include <stdio.h>
#include <pthread.h>

static pthread_cond_t cond = PTHREAD_COND_INITIALIZER;
static pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;

void barrier(volatile int *cnt, const int max, const char *name)
{
	if (pthread_mutex_lock(&lock) != 0)
		goto err;

	/* increment first */
	(*cnt)++;

	if (*cnt == max) {
		if (pthread_cond_broadcast(&cond) != 0)
			goto err;
		goto out;
	} else {
		do {
			if (pthread_cond_wait(&cond, &lock) != 0)
				goto err;
		} while (*cnt < max);
	}
out:
	if (pthread_mutex_unlock(&lock) != 0)
		goto err;

	return;
err:
	perror(name);
	pthread_exit((void *)-1);
}
