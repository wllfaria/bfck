pub static FASM_BOILERPLATE: &str = r#"format ELF64 executable 3
entry _s
Se equ 60
Sw equ 1
C equ 30000
segment writeable
    tape rb C
segment executable
_i:
    mov byte [ebx], 0
    inc ebx
    loop _i
    ret
_e:
    mov eax, Se
    xor edi, edi
    syscall
_s:
    mov ebx, tape
    mov ecx, C
    call _i
"#;

pub static MULTI_INC: &str = r#"_u:
    inc byte [ebx]
    loop _u
    ret"#;

pub static MULTI_DEC: &str = r#"_d:
    dec byte [ebx]
    loop _d
    ret"#;

pub static MULTI_RIGHT: &str = r#"_r:
    inc ebx
    loop _r
    ret"#;

pub static MULTI_LEFT: &str = r#"_l:
    dec ebx
    loop _l
    ret"#;

pub static WRITE: &str = r#"_w:
    mov eax, Sw
    mov edi, 1
    mov esi, ebx
    mov edx, 1
    syscall
    ret"#;

pub static BREAK: &str = r#"_b:
    ret"#;
