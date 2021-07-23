equ:
        push    rbp
        mov     rbp, rsp
        mov     DWORD PTR [rbp-4], edi
        cmp     DWORD PTR [rbp-4], 1
        jne     .L2
        mov     eax, 0
        jmp     .L3
.L2:
        mov     eax, DWORD PTR [rbp-4]
        imul    eax, DWORD PTR [rbp-4]
.L3:
        pop     rbp
        ret