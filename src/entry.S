.section .text

.option norvc

.type start, @function
.global start
start:
        .cfi_startproc

        # First we load the global_pointer
.option push
.option norelax
        la gp, global_pointer
.option pop

        # We ensure that paging is disabled
        csrw satp, zero

        # We setup the stack
        la sp, stack_top

        tail kmain

        .cfi_endproc

.type trap_entry, @function
.global trap_entry
.align 4
trap_entry:
        addi sp, sp, -17*4

        sw ra, 0*8(sp)
        sw a0, 1*8(sp)
        sw a1, 2*8(sp)
        sw a2, 3*8(sp)
        sw a3, 4*8(sp)
        sw a4, 5*8(sp)
        sw a5, 6*8(sp)
        sw a6, 7*8(sp)
        sw a7, 8*8(sp)
        sw t0, 9*8(sp)
        sw t1, 10*8(sp)
        sw t2, 11*8(sp)
        sw t3, 12*8(sp)
        sw t4, 13*8(sp)
        sw t5, 14*8(sp)
        sw t6, 15*8(sp)

        jal ktrap_entry

        lw ra, 0*8(sp)
        lw a0, 1*8(sp)
        lw a1, 2*8(sp)
        lw a2, 3*8(sp)
        lw a3, 4*8(sp)
        lw a4, 5*8(sp)
        lw a5, 6*8(sp)
        lw a6, 7*8(sp)
        lw a7, 8*8(sp)
        lw t0, 9*8(sp)
        lw t1, 10*8(sp)
        lw t2, 11*8(sp)
        lw t3, 12*8(sp)
        lw t4, 13*8(sp)
        lw t5, 14*8(sp)
        lw t6, 15*8(sp)

        addi sp, sp, 17*4

        mret

.end
