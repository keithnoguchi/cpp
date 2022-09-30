/* 3.4.0 Semaphore */
void semaphore_acquire(const int max, volatile int *cnt)
{
	for (;;) {
		while (*cnt >= max);
		__sync_fetch_and_add(cnt, 1);
		if (*cnt <= max)
			break;
		__sync_fetch_and_sub(cnt, 1);
	}
}

void semaphore_release(volatile int *cnt)
{
	__sync_fetch_and_sub(cnt, 1);
}
