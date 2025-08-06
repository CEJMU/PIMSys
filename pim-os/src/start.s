
.extern LD_STACK_PTR

// Put a 64-bit value with little endianness.
.macro PUT_64B high, low
    .word \low
    .word \high
.endm

// Create an entry pointing to a next-level table.
.macro TABLE_ENTRY PA, ATTR
    PUT_64B \ATTR, (\PA) + 0x3
.endm

// Create an entry for a 1GB block.
.macro BLOCK_1GB PA, ATTR_HI, ATTR_LO
    PUT_64B \ATTR_HI | ((\PA) >> 32), ((\PA) & 0xC0000000) | \ATTR_LO | 0x1
.endm

// Create an entry for a 2MB block.
.macro BLOCK_2MB PA, ATTR_HI, ATTR_LO
    PUT_64B \ATTR_HI, ((\PA) & 0xFFE00000) | \ATTR_LO | 0x1
.endm

.section .init

.align 12
ttb0_base:
.set ADDR, 0x000
.rept 0x02
BLOCK_1GB (ADDR << 29), 0, 0x740
.set ADDR, ADDR+2
.endr

// Cached normal DRAM region
BLOCK_1GB (ADDR << 29), 0, 0x74C
.set ADDR, ADDR+2

// Non-cached PIM DRAM region
BLOCK_1GB (ADDR << 29), 0, 0x740
.set ADDR, ADDR+2

// Map rest of Page Table to avoid undefined behavior
.rept 0x3C
BLOCK_1GB (ADDR << 29), 0, 0x74C
.set ADDR, ADDR+2
.endr

.globl _start
_start:
    ldr     x30, =LD_STACK_PTR
    mov     sp, x30

    // Initialize translation table control registers
    ldr x1, =0x13520 // 64GB space 4KB granularity Inner-shareable. Normal Inner and Outer Cacheable.
    msr tcr_el3, x1

    ldr x1, =0xFF440400
    msr mair_el3, x1 // ATTR0 Device-nGnRnE ATTR1 Device. ATTR2 Normal Non-Cacheable. ATTR3 Normal Cacheable.

    bl set_page_table

    // Enable MMU and caches
    mrs x0, sctlr_el3
    orr x0, x0, #(0x1 << 2) // The C bit (data cache).
    orr x0, x0, #(0x1 << 12) // The I bit (instruction cache).
    orr x0, x0, #0x1 // The M bit (MMU).
    msr sctlr_el3, x0
    dsb sy
    isb

    bl entry

.globl set_page_table
set_page_table:
    adr x0, ttb0_base
    msr ttbr0_el3, x0
    tlbi alle3
    isb
    ret
