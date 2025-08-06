MEMORY
{
    bootmem : ORIGIN = 0x0, LENGTH = 0x100000
    dram : ORIGIN = 0x80000000, LENGTH = 0x40000000
    dram_pim_config : ORIGIN = 0xC0000000, LENGTH = 0x4000
    dram_pim_data : ORIGIN = 0xC0004000, LENGTH = 0x3FFFC000
}

ENTRY(_start)
SECTIONS
{
    .init : { *(.init) } > bootmem
    .text : { KEEP(*(.text)) } > dram
    .data : { *(.data) } > dram
    .rodata : { *(.rodata) } > dram
    .bss : { *(.bss) } > dram

    . = ALIGN(8);
    . = . + 0x10000000; # 100 MiB Stack
    LD_STACK_PTR = .;

    .pim_config : { KEEP(*(.pim_config)) } > dram_pim_config
    .pim_data : { KEEP(*(.pim_data)) } > dram_pim_data
}
