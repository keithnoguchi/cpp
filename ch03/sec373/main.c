/* 3.7.3 Benchmark */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <unistd.h>
#include <pthread.h>

/* in lib.c */
extern int noop_lock(const char *name);
extern int noop_unlock(const char *name);
extern int mutex_lock(const char *name);
extern int mutex_unlock(const char *name);
extern int rdlock_lock(const char *name);
extern int rdlock_unlock(const char *name);
extern int rwlock_lock(const char *name);
extern int rwlock_unlock(const char *name);
extern int barrier(volatile int *count, const int max, const char *name);

#define NR_WORKERS 10
#define NR_TIMERS 1
#define NR_LOCK_HOLD_TIME 10
#define RUNTIME_IN_SEC 2

/* Flag to indicate the benchmark runtime. */
volatile bool expired = false;
volatile int start_barrier = 0;
volatile int finish_barrier = 0;

/* How many times those workers run in loop */
static uint64_t counts[NR_WORKERS];

/* lock/unlock dispatcher set in config function */
static int (*lock)(const char *name);
static int (*unlock)(const char *name);

static inline int64_t run(const char *name)
{
	int64_t count = 0;
	while (!expired) {
		if ((*lock)(name) != 0)
			goto err;
		for (int64_t hold = 0; hold < NR_LOCK_HOLD_TIME; hold++)
			asm volatile("nop");
		if ((*unlock)(name) != 0)
			goto err;
		count++;
	}
	return count;
err:
	(*unlock)(name);
	return -1;
}

static inline int wait_start(const char *name)
{
	/* make sure everyone is ready, including the timer worker */
	return barrier(&start_barrier, NR_WORKERS + NR_TIMERS, name);
}

static inline int wait_finish(const char *name)
{
	/* wait only for the workers */
	return barrier(&finish_barrier, NR_WORKERS, name);
}

static void *worker(void *arg)
{
	intptr_t id = (intptr_t)arg;
	char name[BUFSIZ];
	int64_t count;

	snprintf(name, sizeof(name), "worker%ld", id);
	if (wait_start(name) != 0)
		goto err;
	count = run(name);
	if (count < 0)
		goto err;
	if (wait_finish(name) != 0)
		goto err;

	counts[id] = count;
	return (void *)id;
err:
	perror(name);
	return (void *)-1;
}

static void *timer(void *arg)
{
	const char *type = (char *)arg;
	char name[BUFSIZ];

	snprintf(name, sizeof(name), "benchmark-%s", type);
	printf("%s\n", name);

	if (wait_start(name) != 0)
		goto err;
	sleep(RUNTIME_IN_SEC);
	expired = true;
	if (wait_finish(name) != 0)
		goto err;

	for (uint64_t worker = 0; worker < NR_WORKERS; worker++)
		printf("worker%ld: %ld\n", worker, counts[worker]);

	return (void *)type;
err:
	perror(name);
	return (void *)NULL;
}

static const char *config(const int argc, const char *const argv[])
{
	const char *const progname = argv[0];
	if (argc == 1) {
		lock = rdlock_lock;
		unlock = rdlock_unlock;
		return "rdlock";
	}
	switch (argv[1][0]) {
	case 'm':
		lock = mutex_lock;
		unlock = mutex_unlock;
		return "mutex";
	case 'n':
		lock = noop_lock;
		unlock = noop_unlock;
		return "noop";
	case 'r':
		switch (argv[1][1]) {
		case 'd':
			lock = rdlock_lock;
			unlock = rdlock_unlock;
			return "rdlock";
		case 'w':
			lock = rwlock_lock;
			unlock = rwlock_unlock;
			return "rwlock";
		}
		break;
	default:
		break;
	}
	fprintf(stderr, "%s [mutex|noop|rdlock]\n", progname);
	exit(EXIT_FAILURE);
}

int main(int argc, const char *const argv[])
{
	const char *progname = argv[0];
	const char *type = config(argc, argv);
	pthread_t th;
	char *ret;

	for (intptr_t id = 0; id < NR_WORKERS; id++) {
		if (pthread_create(&th, NULL, worker, (void *)id) != 0)
			goto err;
		if (pthread_detach(th) != 0)
			goto err;
	}

	/* benchmark report thread. */
	if (pthread_create(&th, NULL, timer, (void *)type) != 0)
		goto err;
	if (pthread_join(th, (void **)&ret) != 0)
		goto err;
	if (ret != type) {
		fprintf(stderr, "timer0: error (%p)\n", ret);
		goto err;
	}
	exit(EXIT_SUCCESS);
err:
	perror(progname);
	exit(EXIT_FAILURE);
}
