/* 3.2.2 Test And Set (TAS) */
#include <stdbool.h>

bool test_and_set_not(bool *p)
{
	if (*p)
		return true;
	else {
		*p = true;
		return false;
	}
}
