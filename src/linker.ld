ENTRY(start);

MEMORY {
  kernel_space (rwx) : ORIGIN = 0x80200000, LENGTH = 1024M
}

SECTIONS {
  .text : ALIGN(4K) {
    *(.text);
  } > kernel_space

  .bss : ALIGN(4K) {
    PROVIDE(bss_start = .);
    *(.bss);
    . += 4096;
    PROVIDE(stack_top = .);
    . += 4096;
    PROVIDE(global_pointer = .);
    PROVIDE(bss_end = .);
  } > kernel_space

  .rodata : ALIGN(4K) {
    *(.rodata);
  } > kernel_space

  .data : ALIGN(4K) {
    *(.data);
  } > kernel_space

  .eh_frame : ALIGN(4K) {
    *(.eh_frame);
  } > kernel_space

  .riscv.attributes : {
    *(.riscv.attributes);
  } > kernel_space

  PROVIDE(kernel_end = .);
}
