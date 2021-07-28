bits 64
default rel
section .text

global mymain:
mymain:
	push rbp
	mov rbp, rsp
%line 1 "mymain.rs"
	mov eax, 1
	mov ebx, 1
	cmp eax, ebx
%line 2 "mymain.rs"
	jne .L0		; Jump if not equal
	mov ebx, 11
	mov ebx, 12
	add eax, ebx
%line 3 "mymain.rs"
	call keribeth
%line 4 "mymain.rs"
.L0:
	pop rbp
%line 5 "mymain.rs"
	ret
%line 6 "mymain.rs"
global keribeth:
keribeth:
	push rbp
	mov rbp, rsp
%line 8 "mymain.rs"
	mov eax, 39
	mov ebx, 2
	add eax, ebx
%line 9 "mymain.rs"
	pop rbp
%line 10"mymain.rs"
	ret
