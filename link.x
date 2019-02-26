/* Memory layout */
/* 1K = 1 KiBi = 1024 bytes */

INPUT("32MX470F512H_procdefs.ld");

/* The entry point is the reset handler */
ENTRY(Reset);

EXTERN(RESET_VECTOR);

/* stack */
PROVIDE(_stack = ORIGIN(kseg1_data_mem) + LENGTH(kseg1_data_mem));


/* # Pre-initialization function */
/* If the user overrides this using the `pre_init!` macro or by creating a `__pre_init` function,
   then the function this points to will be called before the RAM is initialized. */
PROVIDE(__pre_init = DefaultPreInit);

/* # Sections */
SECTIONS
{
  /* Boot Sections */
  .reset :
  {
    KEEP(*(.reset))
    KEEP(*(.reset.startup))
  } > kseg1_boot_mem


  /* PROVIDE(_stext = ADDR(.vector_table) + SIZEOF(.vector_table)); */

  /* ### .text */
  .text  :
  {
    *(.text .text.*);
  } > kseg0_program_mem
  
  .got :
  {
    *(.got);
  } > kseg0_program_mem

  /* ### .rodata */
  .rodata : ALIGN(4)
  {
    *(.rodata .rodata.*);

    /* 4-byte align the end (VMA) of this section.
       This is required by LLD to ensure the LMA of the following .data
       section will have the correct alignment. */
    . = ALIGN(4);
  } > kseg0_program_mem

  /* ## Sections in RAM */
  /* ### .data */
  .data : ALIGN(4)
  {
    *(.data .data.*);

    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
  } > kseg1_data_mem AT > kseg0_program_mem

  /* VMA of .data */
  __sdata = ADDR(.data);
  __edata = ADDR(.data) + SIZEOF(.data);

  /* LMA of .data */
  __sidata = LOADADDR(.data);

  /* ### .bss */
  .bss : ALIGN(4)
  {
    *(.bss .bss.*);

    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
  } > kseg1_data_mem

  __sbss = ADDR(.bss);
  __ebss = ADDR(.bss) + SIZEOF(.bss);

  /* Place the heap right after `.bss` */
  __sheap = ADDR(.bss) + SIZEOF(.bss);

  /* Stack usage metadata emitted by LLVM */
  .stack_sizes (INFO) :
  {
    KEEP(*(.stack_sizes));
  }

  /* ## Discarded sections */
  /DISCARD/ :
  {
    *(.reginfo);
    *(.MIPS.abiflags);
  }
}

