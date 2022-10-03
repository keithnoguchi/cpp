/* 3.6.1 Memory Barrier by Spinlock */

void barrier(volatile int *cnt, const int max)
{
	__sync_fetch_and_add(cnt, 1);
	while (*cnt < max); /* spin! */
}
