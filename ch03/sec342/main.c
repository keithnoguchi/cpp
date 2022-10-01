/* 3.4.2 Posix Semaphore */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <unistd.h>
#include <pthread.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <semaphore.h>

#define NR_THREADS 20000

/* retry sem_open() in case there is a stable semaphore */
#define NR_SEM_OPEN 5
#define SEM_OPEN_SLEEP_US 10000 /* 10ms */

/* worker defined in lib.c */
extern void *worker(void *arg);

/* global variables, shared by workers in lib.c */
const unsigned int semaphore_count = 3;
const char *semaphore_name = "/cpr-sec342-semaphore";
uint64_t counter;

int main(int argc, const char *const argv[])
{
	const char *progname = argv[0];
	pthread_t *p, *workers;
	sem_t *sem = NULL;
	intptr_t id;

	workers = malloc(sizeof(pthread_t) * NR_THREADS);
	if (!workers)
		goto err;

	for (int i = 0; i < NR_SEM_OPEN; i++) {
		sem = sem_open(semaphore_name, O_CREAT, 0666, semaphore_count);
		if (sem)
			break;
		/* let's retry after the sleep */
		usleep(SEM_OPEN_SLEEP_US * i);
	}
	if (!sem)
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
		if (got < 0)
			goto err;
	}
	if (sem_close(sem) != 0)
		goto err;
	if (sem_unlink(semaphore_name) != 0)
		goto err;
	free(workers);
	exit(EXIT_SUCCESS);
err:
	perror(progname);
	if (sem) {
		/* no error check here... */
		sem_close(sem);
		sem_unlink(semaphore_name);
	}
	if (workers)
		free(workers);
	exit(EXIT_FAILURE);
}
