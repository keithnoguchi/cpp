	.file	"test_and_set.c"
	.text
	.p2align 4
	.globl	test_and_set
	.type	test_and_set, @function
test_and_set:
.LFB0:
	.cfi_startproc
	movl	$1, %eax
	xchgb	(%rdi), %al
	testb	%al, %al
	setne	%al
	ret
	.cfi_endproc
.LFE0:
	.size	test_and_set, .-test_and_set
	.ident	"GCC: (GNU) 12.1.0"
	.section	.note.GNU-stack,"",@progbits
