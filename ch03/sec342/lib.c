/* 3.4.2 Posix Semaphore */
#include <stdio.h>
#include <stdint.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <semaphore.h>
#include <errno.h>
#include <assert.h>

#define WORKER_SLEEP_US 1 /* 1us */

/* global variable defined in main.c */
extern const unsigned int semaphore_count;
extern const char *semaphore_name;
extern uint64_t counter;

void *worker(void *arg)
{
	intptr_t id = (intptr_t)arg;
	sem_t *sem = NULL;

	sem = sem_open(semaphore_name, 0);
	if (!sem)
		goto err;
	if (sem_wait(sem) != 0)
		goto err;
	__sync_fetch_and_add(&counter, 1);
	/* releasing the CPU by sleeping 1us to cause an issue
	 * with out sem_wait call above. */
	usleep(WORKER_SLEEP_US);
	if (counter > semaphore_count) {
		/* counter shouldn't be more than the semaphore count. */
		goto err;
	}
	__sync_fetch_and_sub(&counter, 1);
	if (sem_post(sem) != 0)
		goto err;
	if (sem_close(sem) != 0)
		goto err;

	return (void *)id;
err:
	{
		char name[BUFSIZ];
		snprintf(name, sizeof(name) - 1, "worker%ld", id);
		if (errno)
			perror(name);
		else {
		    fprintf(stdout, "%s: counter = %ld\n", name, counter);
		    fflush(stdout);
		}
	}
	if (sem)
		sem_close(sem);
	/* generic error code pass back to the main thread. */
	return (void *)-1;
}
