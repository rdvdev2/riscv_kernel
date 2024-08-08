.section .init

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

.end
