/* 3.3.1 Spinlock by TTAS */
#include <stdbool.h>

static inline bool test_and_set(volatile bool *lock)
{
	return __sync_lock_test_and_set(lock, true);
}

static inline void tas_release(bool *lock)
{
	__sync_lock_release(lock);
}

/* TTAS: Test and Test And Set */
static inline void ttas_spinlock_aquire(volatile bool *lock)
{
	for (;;) {
		while (*lock);
		if(!test_and_set(lock))
			break;
	}
}

/* TAS: Test And Set */
static inline void tas_spinlock_aquire(volatile bool *lock)
{
	while (test_and_set(lock));
}

void spinlock_aquire(volatile bool *lock)
{
	return ttas_spinlock_aquire(lock);
}

void spinlock_release(bool *lock)
{
	tas_release(lock);
}
