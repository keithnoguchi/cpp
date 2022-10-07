/* 3.7.2 Pthread Read/Write Lock */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <pthread.h>
#include <assert.h>

/* in lib.c */
extern void *reader(void *arg);
extern void *writer(void *arg);

#define NR_READERS 20
#define NR_WRITERS 200
pthread_rwlock_t lock = PTHREAD_RWLOCK_INITIALIZER;
const int nr_reader_loop = 2000;
const int nr_writer_loop = 1000;
uint64_t counter = 0;

int main(int argc, const char *const argv[])
{
	const char *progname = argv[0];
	pthread_t *p, *readers = NULL, *writers = NULL;
	intptr_t id;

	readers = malloc(sizeof(pthread_t) * NR_READERS);
	if (!readers)
		goto err;
	writers = malloc(sizeof(pthread_t) * NR_WRITERS);
	if (!writers)
		goto err;

	for (p = readers, id = 0; id < NR_READERS; p++, id++)
		if (pthread_create(p, NULL, reader, (void *)id) != 0)
			goto err;
	for (p = writers, id = 0; id < NR_WRITERS; p++, id++)
		if (pthread_create(p, NULL, writer, (void *)id) != 0)
			goto err;

	printf("%s\n", progname);

	for (p = writers, id = 0; id < NR_WRITERS; p++, id++) {
		intptr_t ret;
		if (pthread_join(*p, (void **)&ret) != 0)
			goto err;
		if (ret != id)
			fprintf(stderr, "%s: writer%ld error\n",
				progname, id);
	}
	for (p = readers, id = 0; id < NR_READERS; p++, id++) {
		intptr_t ret;
		if (pthread_join(*p, (void **)&ret) != 0)
			goto err;
		if (ret != id)
			fprintf(stderr, "%s: reader%ld error\n",
				progname, id);
	}
	assert(counter == NR_WRITERS * nr_writer_loop);
	if (pthread_rwlock_destroy(&lock) != 0)
		goto err;
	free(writers);
	free(readers);
	exit(EXIT_SUCCESS);
err:
	perror(progname);
	/* this could be ended up as a double destroy */
	pthread_rwlock_destroy(&lock);
	if (readers)
		free(readers);
	if (writers)
		free(writers);
	exit(EXIT_FAILURE);
}
