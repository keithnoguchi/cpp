/* SPDX-License-Identifier: GPL-2.0 */
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <pthread.h>

#define NUM_THREADS 5

static void *worker(void *arg)
{
	intptr_t id = (intptr_t)arg;
	for (int i = 0; i < 5; i++) {
		printf("id = %ld, i = %d\n", id, i);
		sleep(1);
	}
	return "finished";
}

int main(int argc, char *argv[])
{
	pthread_t workers[NUM_THREADS];

	/* fork threads */
	for (intptr_t i = 0; i < NUM_THREADS; i++) {
		int ret = pthread_create(&workers[i], NULL, worker, (void *)i);
		if (ret != 0) {
			perror("pthread_create");
			return EXIT_FAILURE;
		}
	}

	/* join threads */
	for (int i = 0; i < NUM_THREADS; i++) {
		char *ret;
		if (pthread_join(workers[i], (void *)&ret) != 0) {
			perror("pthread_join");
		} else {
			printf("worker%d: msg = %s\n", i, ret);
		}
	}
	return EXIT_SUCCESS;
}
