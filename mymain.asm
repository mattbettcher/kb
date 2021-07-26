bits 64
default rel
section .text

global mymain:
mymain:
	push rbp
	mov rbp, rsp
	mov eax, 1
	mov ebx, 0
	cmp eax, ebx
	jne .L0		; Jump if not equal
	mov ebx, 0
.L0:
	pop rbp
	ret
