; x86_64 부트 로더

; 섹션 .text를 정의합니다.
.section .text

; 시작 부분에 _start 레이블을 정의합니다.
.global _start
_start:

; 스택을 초기화합니다.
movq $0x7c00, %rsp

; 64비트 모드로 전환합니다.
mov $0x8000, %eax
mov %eax, %cr3
mov %cr0, %eax
or $0x80000000, %eax
mov %eax, %cr0

; Rust 코드를 호출합니다.
call rust_main

; 무한 루프를 실행합니다.
loop:
    jmp loop
