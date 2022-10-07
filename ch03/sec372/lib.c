/* 3.7.2 Posix Read/Write Lock */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <pthread.h>

/* in main.c */
extern uint64_t counter;
extern const int nr_reader_loop;
extern const int nr_writer_loop;
extern pthread_rwlock_t lock;

/* simple sampled logging */
static inline void sampled_log(const char *name, const uint64_t counter)
{
#define LOG_SAMPLE_RAND_MODULO 0x2f
	static unsigned int log_sample_rand_seed = 0;

	if (rand_r(&log_sample_rand_seed) % LOG_SAMPLE_RAND_MODULO == 0)
		printf("%s: current_counter=%ld\n", name, counter);
}

void *reader(void *arg)
{
	intptr_t id = (intptr_t)arg;
	char name[BUFSIZ];
	volatile uint64_t current_counter;

	for (int i = 0; i < nr_reader_loop; i++) {
		if (pthread_rwlock_rdlock(&lock) != 0)
			goto err;
		current_counter = counter;
		if (pthread_rwlock_unlock(&lock) != 0)
			goto err;
	}
	snprintf(name, sizeof(name), "reader%ld", id);
	sampled_log(name, current_counter);

	return (void *)id;
err:
	perror(name);
	return (void *)-1;
}

void *writer(void *arg)
{
	intptr_t id = (intptr_t)arg;
	char name[BUFSIZ];
	volatile uint64_t current_counter;

	for (int i = 0; i < nr_writer_loop; i++) {
		if (pthread_rwlock_wrlock(&lock) != 0)
			goto err;
		current_counter = ++counter;
		if (pthread_rwlock_unlock(&lock) != 0)
			goto err;
	}
	snprintf(name, sizeof(name), "writer%ld", id);
	sampled_log(name, current_counter);

	return (void *)id;
err:
	perror(name);
	return (void *)-1;
}
