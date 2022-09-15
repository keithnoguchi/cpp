	.file	"wait_while.c"
	.text
	.p2align 4
	.globl	wait_while
	.type	wait_while, @function
wait_while:
.LFB0:
	.cfi_startproc
	movl	(%rdi), %eax
	testl	%eax, %eax
	jne	.L1
.L3:
	jmp	.L3
	.p2align 4,,10
	.p2align 3
.L1:
	ret
	.cfi_endproc
.LFE0:
	.size	wait_while, .-wait_while
	.ident	"GCC: (GNU) 12.1.0"
	.section	.note.GNU-stack,"",@progbits
