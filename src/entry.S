.section .init

.option norvc

.type start, @function
.global start
start:
        .cfi_startproc

.option push
.option norelax
        la gp, global_pointer
.option pop

        csrw satp, zero

        la sp, stack_top

        la t5, bss_start
        la t6, bss_end
bss_clear:
        sd zero, (t5)
        addi t5, t5, 8
        bltu t5, t6, bss_clear

        # Setup MPP (M-Mode previous privilege)
        li t2, 0x1800
        csrc mstatus, t2
        li t3, 0x800
        csrs mstatus, t3
        # Setup MEPC (M-Mode Exception PC)
        la t0, kmain
        csrw mepc, t0

        tail kmain

        .cfi_endproc

.end
