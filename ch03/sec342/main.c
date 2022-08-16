/* SPDX-License-Identifier: GPL-2.0 */
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <pthread.h>
#include <semaphore.h>

#define NUM_THREADS	10
#define NUM_LOOP	10

int count = 0;

/* worker thread */
static void *worker(void *arg)
{
	sem_t *s = sem_open("/mysemaphore", 0);
	if (s == SEM_FAILED) {
		perror("sem_open");
		exit(1);
	}

	for (int i = 0; i < NUM_LOOP; i++) {
		if (sem_wait(s) == -1) {
			perror("sem_wait");
			exit(1);
		}

		/* increment the counter atomically. */
		__sync_fetch_and_add(&count, 1);
		printf("count = %d\n", count);

		/* 10ms sleep */
		usleep(10000);

		/* decrement the counter atomically */
		__sync_fetch_and_sub(&count, 1);

		if (sem_post(s) == -1) {
			perror("sem_post");
			exit(1);
		}
	}

	if (sem_close(s) == -1)
		perror("sem_close");

	return NULL;
}

int main()
{
	/* semaphore with 3 */
	sem_t *s = sem_open("/mysemaphore", O_CREAT, 0660, 3);
	if (s == SEM_FAILED) {
		perror("sem_open");
		goto err;
	}

	/* initial count */
	printf("count = %d\n", count);

	/* create workers */
	pthread_t workers[NUM_THREADS];
	for (int i = 0; i < NUM_THREADS; i++) {
		int ret = pthread_create(&workers[i], NULL, worker, NULL);
		if (ret == -1) {
			perror("pthread_create");
			goto err;
		}
	}

	/* wait for all the workers to complete */
	for (int i = 0; i < NUM_THREADS; i++) {
		/* ignore the return value */
		pthread_join(workers[i], NULL);
	}

	/* final count, this should be zero always */
	printf("count = %d\n", count);

	if (sem_close(s) == -1) {
		perror("sem_close");
		goto err;
	}

	if (sem_unlink("/mysemaphore") == -1) {
		perror("sem_unlink");
		goto err;
	}

	return EXIT_SUCCESS;
err:
	return EXIT_FAILURE;
}
