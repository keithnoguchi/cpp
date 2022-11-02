/* 4.4.0 Recursive Lock */
#include <assert.h>

#include "lib.h"

static inline void spinlock_acquire(volatile bool *lock);
static inline void spinlock_release(volatile bool *lock);

void recursive_lock_acquire(struct recursive_lock *lock, const int id)
{
	assert(id != INIT_ID);
	if (lock->lock && lock->id == id)
		lock->cnt++;
	else {
		spinlock_acquire(&lock->lock);
		lock->id = id;
		lock->cnt++;
	}
}

void recursive_lock_release(struct recursive_lock *lock, const int id)
{
	assert(lock->id == id);
	lock->cnt--;
	if (lock->cnt)
		return;
	lock->id = INIT_ID;
	spinlock_release(&lock->lock);
}

static inline void spinlock_acquire(volatile bool *lock)
{
	/* TTAS */
	for (;;) {
		while (*lock);
		if (!__sync_lock_test_and_set(lock, 1))
			break;
	}
}

static inline void spinlock_release(volatile bool *lock)
{
	__sync_lock_release(lock);
}
