/* 4.4.0 Recursive Lock */
#ifndef _LOCK_H
#define _LOCK_H

/* ID == 0 is reserved and set through the lock BSS declaration */
#define INIT_ID 0

#include <stdbool.h>

struct recursive_lock {
	bool lock;
	int id;
	int cnt;
};

extern void recursive_acquire(struct recursive_lock *lock, const int id);
extern void recursive_release(struct recursive_lock *lock, const int id);

#endif /* _LOCK_H */
