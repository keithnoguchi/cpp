	.file	"compare_and_swap.c"
	.text
	.p2align 4
	.globl	compare_and_swap
	.type	compare_and_swap, @function
compare_and_swap:
.LFB0:
	.cfi_startproc
	movq	%rsi, %rax
	lock cmpxchgq	%rdx, (%rdi)
	sete	%al
	ret
	.cfi_endproc
.LFE0:
	.size	compare_and_swap, .-compare_and_swap
	.ident	"GCC: (GNU) 12.1.0"
	.section	.note.GNU-stack,"",@progbits
