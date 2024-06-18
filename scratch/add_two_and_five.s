; this is the hand-written assembly representation of the `add_2_and_5.bf`
; available at `samples/add_2_and_5.bf`
; it is heavily commented so I don't forget what it does LOL

format ELF64 executable 3
entry start

SYS_exit equ 60
SYS_write equ 1
CAPACITY equ 32

segment writeable
    tape rb CAPACITY                ; reserving space for the tapy with `CAPACITY`

segment executable
write:                              ; prints out the current cell with write syscall
    mov eax, SYS_write
    mov edi, 1                      ; 1 is stdout
    mov esi, ebx
    mov edx, 1                      ; we always only print one byte
    syscall
    ret                             ; give back control to whoever calls this

init_tape:                          ; initializes the tape to be 0 filled
    mov byte [ebx], 0
    inc ebx
    loop init_tape                  ; loop automatically decrements rcx
    ret

inc_ptr:                            ; little optimization to increment more than once
    inc byte [ebx]
    loop inc_ptr                    ; the magic here is to increment ebx up to rcx times
    ret

brk:                                ; this is just a break;
    ret

start:                              ; program entrypoint
    mov ebx, tape                   ; ebx is out data_ptr, so we set it to the tape start
    mov ecx, CAPACITY               ; setting ecx to CAPACITY to initialize the tape
    call init_tape

    mov ecx, 2                      ; ++ incrementing the current cell by 2
    call inc_ptr

    inc ebx                         ; > moving to next cell
    mov ecx, 5                      ; +++++ incrementing the cell by 5
    call inc_ptr

    call j1z                        ; [ first jump if zero

    mov ecx, 8                      ; ++++++++ if not zero, we go here, also at the end
    call inc_ptr                    ; and we are just adding 8 to the cell we points

    call j2z                        ; [ second jump if zero
    dec ebx                         ; < move pointer left
    call write                      ; . write cell
    jmp exit                        ; exit happy

; jumps are implemented as 2 blocks, one for the comparation
; and another separate one so we can loop when needed the idea
; is to have all the "body" of the jump inside of the body
j1z:
    cmp byte [ebx], 0               ; comparing against 0
    je brk                          ; since we are in a jump if zero, we break if 0
    jne j1b                         ; else we go into the body

j1b:                                ; body of the first jump
    dec ebx                         ; < move left
    inc byte [ebx]                  ; + increment cell at pointer
    inc rbx                         ; > move pointer right
    dec byte [ebx]                  ; - decrement cell at pointer
    cmp byte [ebx], 0               ; ] jump if not zero
    jne j1b                         ; go back if not zero and loop
    je brk                          ; if zero break

j2z:
    cmp byte [ebx], 0               ; comparing against 0
    je brk                          ; if zero we break
    jne j2b                         ; else go to body

j2b:
    dec ebx                         ; < move left
    mov ecx, 6                      ; ++++++ increment 6 times
    call inc_ptr
    inc ebx                         ; > move right
    dec byte [ebx]                  ; - decrement a single time
    cmp byte [ebx], 0               ; ] jump if not zero
    je brk                          ; when zero we break
    jne j2b                         ; when non zero loop


exit:                               ; exit syscall (60) with status code (0) success
    mov eax, SYS_exit
    xor edi, edi
    syscall
