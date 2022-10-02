/* 3.5.0 Posix Condition Variables */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <pthread.h>

#define NR_PRODUCERS 1
#define NR_CONSUMERS 100

/* in lib.c */
extern void *producer(void *arg);
extern void *consumer(void *arg);

int main(int argc, const char *const argv[])
{
	const char *progname = argv[0];
	pthread_t *p, *producers = NULL, *consumers = NULL;
	intptr_t id;

	producers = malloc(sizeof(pthread_t) * NR_PRODUCERS);
	if (!producers)
		goto err;
	consumers = malloc(sizeof(pthread_t) * NR_CONSUMERS);
	if (!consumers)
		goto err;

	/* let's do it */
	for (p = producers, id = 0; id < NR_PRODUCERS; p++, id++)
		if (pthread_create(p, NULL, producer, (void *)id) != 0)
			goto err;
	for (p = consumers, id = 0; id < NR_CONSUMERS; p++, id++)
		if (pthread_create(p, NULL, consumer, (void *)id) != 0)
			goto err;

	/* wait for the completion. */
	for (p = consumers, id = 0; id < NR_CONSUMERS; p++, id++) {
		intptr_t ret;
		if (pthread_join(*p, (void **)&ret) != 0)
			goto err;
		if (ret != id)
			goto err;
	}
	for (p = producers, id = 0; id < NR_PRODUCERS; p++, id++) {
		intptr_t ret;
		if (pthread_join(*p, (void **)&ret) != 0)
			goto err;
		if (ret != id)
			goto err;
	}
	printf("%s\n", progname);
	free(consumers);
	free(producers);

	exit(EXIT_SUCCESS);
err:
	perror(progname);
	if (consumers)
		free(consumers);
	if (producers)
		free(producers);

	exit(EXIT_FAILURE);
}
