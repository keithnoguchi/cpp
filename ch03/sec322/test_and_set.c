/* 3.2.2 Test And Set (TAS) */
#include <stdbool.h>

bool test_and_set(volatile bool *p)
{
	return __sync_lock_test_and_set(p, true);
}
