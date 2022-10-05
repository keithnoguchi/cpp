/* 3.7.1 Readers/Writer Lock by Spinlock */
#include <stdbool.h>

/* in spinlock.c */
extern void spinlock_aquire(volatile bool *lock);
extern void spinlock_release(volatile bool *lock);

void rwlock_read_aquire(int *rcnt, volatile int *wcnt)
{
	for (;;) {
		while (*wcnt);
		__sync_fetch_and_add(rcnt, 1);
		if (!*wcnt)
			return;
		__sync_fetch_and_sub(rcnt, 1);
	}
}

void rwlock_read_release(int *rcnt)
{
	__sync_fetch_and_sub(rcnt, 1);
}

void rwlock_write_aquire(bool *lock, volatile int *rcnt, int *wcnt)
{
	__sync_fetch_and_add(wcnt, 1);
	while (*rcnt);
	spinlock_aquire(lock);
}

void rwlock_write_release(bool *lock, int *wcnt)
{
	spinlock_release(lock);
	__sync_fetch_and_sub(wcnt, 1);
}
