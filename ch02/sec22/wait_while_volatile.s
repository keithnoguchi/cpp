	.file	"wait_while_volatile.c"
	.text
	.p2align 4
	.globl	wait_while_volatile
	.type	wait_while_volatile, @function
wait_while_volatile:
.LFB0:
	.cfi_startproc
	.p2align 4,,10
	.p2align 3
.L2:
	movl	(%rdi), %eax
	testl	%eax, %eax
	je	.L2
	ret
	.cfi_endproc
.LFE0:
	.size	wait_while_volatile, .-wait_while_volatile
	.ident	"GCC: (GNU) 12.1.0"
	.section	.note.GNU-stack,"",@progbits
