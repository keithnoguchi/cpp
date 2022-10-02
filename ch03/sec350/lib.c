/* 3.5.0 Posix Condition Variables */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <stdbool.h>
#include <pthread.h>

/* shared variables between the producers and the consumers */
static pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t cond = PTHREAD_COND_INITIALIZER;
static volatile bool ready = false;
static volatile bool done = false;
static char buf[BUFSIZ];

void *consumer(void *arg)
{
	intptr_t id = (intptr_t)arg;
	char name[BUFSIZ];

	snprintf(name, sizeof(name), "consumer%ld", id);

	/* you need to lock the mutex first */
	if (pthread_mutex_lock(&lock) != 0)
		goto err;

	/* wait for the condition */
	while(!ready) {
		if (pthread_cond_wait(&cond, &lock) != 0)
			goto err;
	}
	/* only one consumer will get it */
	if (done)
		goto done;

	/* you're lucky */
	printf("%s: got '%s'\n", name, buf);
	done = true;
done:
	if (pthread_mutex_unlock(&lock) != 0)
	    goto err;

	return (void *)id;
err:
	perror(name);
	return (void *)-1;
}

void *producer(void *arg)
{
	intptr_t id = (intptr_t)arg;
	char *p, name[BUFSIZ];
	size_t len;

	snprintf(name, sizeof(name), "prodcuer%ld", id);
	printf("%s> ", name);
	p = fgets(buf, sizeof(buf), stdin);
	if (!p)
		goto err;
	len = strlen(p) > 0 ? strlen(p) - 1 : 0;
	buf[len] = '\0'; /* strip off the newline. */

	/* we need to lock first */
	if (pthread_mutex_lock(&lock) != 0)
		goto err;

	/* let every consumers know it's ready */
	ready = true;
	if (pthread_cond_broadcast(&cond) != 0)
		goto err;

	/* now, we're done */
	if (pthread_mutex_unlock(&lock) != 0)
		goto err;

	return (void *)id;
err:
	perror(name);
	return (void *)-1;
}
