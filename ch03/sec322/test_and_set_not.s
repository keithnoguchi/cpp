	.file	"test_and_set_not.c"
	.text
	.p2align 4
	.globl	test_and_set_not
	.type	test_and_set_not, @function
test_and_set_not:
.LFB0:
	.cfi_startproc
	movzbl	(%rdi), %eax
	testb	%al, %al
	jne	.L1
	movb	$1, (%rdi)
.L1:
	ret
	.cfi_endproc
.LFE0:
	.size	test_and_set_not, .-test_and_set_not
	.ident	"GCC: (GNU) 12.1.0"
	.section	.note.GNU-stack,"",@progbits
