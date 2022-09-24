/* SPDX-License-Identifier: GPL-2.0 */
void wait_while_volatile(volatile int *p)
{
	while (*p == 0) {}
}
