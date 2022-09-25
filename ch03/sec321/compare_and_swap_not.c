/* 3.2.1 Compare And Swap (CAS) */
#include <stdint.h>
#include <stdbool.h>

bool compare_and_swap_not(uint64_t *p, uint64_t val, uint64_t newval)
{
	if (*p == val)
		return false;
	*p = newval;
	return true;
}
