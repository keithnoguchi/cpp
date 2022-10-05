/* 3.7.1 Readers/Writer Lock by Spinlock */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <pthread.h>
#include <assert.h>

/* in lib.c */
extern void rwlock_read_aquire(int *rcnt, volatile int *wcnt);
extern void rwlock_read_release(int *rcnt);
extern void rwlock_write_aquire(bool *lock, volatile int *rcnt, int *wcnt);
extern void rwlock_write_release(bool *lock, int *wcnt);

#define NR_READERS 40
#define NR_WRITERS 20
#define NR_READER_LOOP 2000
#define NR_WRITER_LOOP 1000

/* state protected by the readers/writer lock. */
static bool lock;
static int rcount = 0;
static int wcount = 0;
static uint64_t counter = 0;

static void *reader(void *arg)
{
	intptr_t id = (intptr_t)arg;
	uint64_t current_counter;
	char name[BUFSIZ];

	snprintf(name, sizeof(name), "reader%ld", id);

	for (int i = 0; i < NR_READER_LOOP; i++) {
		rwlock_read_aquire(&rcount, &wcount);
		current_counter = counter;
		rwlock_read_release(&rcount);
	}

	/* simple samplling. */
	if (id % NR_READERS == 0)
		printf("%s: current_counter=%ld\n", name, current_counter);

	return (void *)id;
}

static void *writer(void *arg)
{
	intptr_t id = (intptr_t)arg;
	uint64_t current_counter;
	char name[BUFSIZ];

	snprintf(name, sizeof(name), "writer%ld", id);

	for (int i = 0; i < NR_WRITER_LOOP; i++) {
		rwlock_write_aquire(&lock, &rcount, &wcount);
		current_counter = ++counter;
		rwlock_write_release(&lock, &wcount);
	}

	/* simple sampling. */
	if (id % NR_WRITERS == 0)
		printf("%s: counter=%ld\n", name, current_counter);

	return (void *)id;
}

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
		int ret;
		if (pthread_join(*p, (void **)&ret) != 0)
			goto err;
		if (ret != id)
			fprintf(stderr, "writer%ld: error (%d)\n", id, ret);
	}
	for (p = readers, id = 0; id < NR_READERS; p++, id++) {
		int ret;
		if (pthread_join(*p, (void **)&ret) != 0)
			goto err;
		if (ret != id)
			fprintf(stderr, "reader%ld: error (%d)\n", id, ret);
	}
	printf("%s: counter=%ld\n", progname, counter);
	assert(counter == NR_WRITERS * NR_WRITER_LOOP);
	free(writers);
	free(readers);
	exit(EXIT_SUCCESS);
err:
	perror(progname);
	if (writers)
		free(writers);
	if (readers)
		free(readers);
	exit(EXIT_FAILURE);
}
