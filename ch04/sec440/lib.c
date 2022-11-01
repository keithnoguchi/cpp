/* 4.4.0 Recursive Lock */
#include <assert.h>

#include "lib.h"

static inline void spinlock_acquire(volatile bool *lock);
static inline void spinlock_release(volatile bool *lock);

void recursive_acquire(struct recursive_lock *lock, const int id)
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

void recursive_release(struct recursive_lock *lock, const int id)
{
	if (lock->id != id)
		assert(false);
	lock->cnt--;
	lock->id = INIT_ID;
	spinlock_release(&lock->lock);
}

static inline void spinlock_acquire(volatile bool *lock)
{
	return;
}

static inline void spinlock_release(volatile bool *lock)
{
	return;
}
