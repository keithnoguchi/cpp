	.file	"compare_and_swap_not.c"
	.text
	.p2align 4
	.globl	compare_and_swap_not
	.type	compare_and_swap_not, @function
compare_and_swap_not:
.LFB0:
	.cfi_startproc
	xorl	%eax, %eax
	cmpq	%rsi, (%rdi)
	je	.L1
	movq	%rdx, (%rdi)
	movl	$1, %eax
.L1:
	ret
	.cfi_endproc
.LFE0:
	.size	compare_and_swap_not, .-compare_and_swap_not
	.ident	"GCC: (GNU) 12.1.0"
	.section	.note.GNU-stack,"",@progbits
