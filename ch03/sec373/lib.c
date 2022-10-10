/* 3.7.3 Benchmark */
#include <stdio.h>
#include <pthread.h>

static pthread_mutex_t _mutex_lock = PTHREAD_MUTEX_INITIALIZER;
static pthread_rwlock_t _rwlock_lock = PTHREAD_RWLOCK_INITIALIZER;
static pthread_mutex_t barrier_lock = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t barrier_cond = PTHREAD_COND_INITIALIZER;

int noop_lock(const char *name)
{
	return 0;
}

int noop_unlock(const char *name)
{
	return 0;
}

int mutex_lock(const char *name)
{
	return pthread_mutex_lock(&_mutex_lock);
}

int mutex_unlock(const char *name)
{
	return pthread_mutex_unlock(&_mutex_lock);
}

int rdlock_lock(const char *name)
{
	return pthread_rwlock_rdlock(&_rwlock_lock);
}

int rdlock_unlock(const char *name)
{
	return pthread_rwlock_unlock(&_rwlock_lock);
}

int rwlock_lock(const char *name)
{
	return pthread_rwlock_wrlock(&_rwlock_lock);
}

int rwlock_unlock(const char *name)
{
	return pthread_rwlock_unlock(&_rwlock_lock);
}

int barrier(volatile int *counter, const int max, const char *name)
{
	if (pthread_mutex_lock(&barrier_lock) != 0)
		goto err;

	++*counter;
	if (*counter == max) {
		/* wake up everyone */
		if (pthread_cond_broadcast(&barrier_cond) != 0)
			goto err;
	} else {
		if (pthread_cond_wait(&barrier_cond, &barrier_lock) != 0)
			goto err;
	}
	if (pthread_mutex_unlock(&barrier_lock) != 0)
		goto err;

	return 0;
err:
	perror(name);
	/* ignore error */
	pthread_mutex_unlock(&barrier_lock);
	return -1;
}
