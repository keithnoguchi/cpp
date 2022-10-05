/* 3.7.1 Readers/Writer Lock by Spinlock */
#include <stdbool.h>

void spinlock_aquire(volatile bool *lock)
{
	for (;;) {
		/* TTAS: Test Test And Set. */
		while (*lock);
		if (!__sync_lock_test_and_set(lock, true))
			break;
	}
}

void spinlock_release(volatile bool *lock)
{
	__sync_lock_release(lock);
}
