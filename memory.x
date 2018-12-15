MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x00004000, LENGTH = 0xC000
  RAM : ORIGIN = 0x20000000, LENGTH = 8K
}

__app_start__ = ORIGIN(FLASH);
__app_end__   = __app_start__ + LENGTH(FLASH);
__ram_start__ = ORIGIN(RAM);
__ram_size__  = LENGTH(RAM);
__ram_end__   = __ram_start__ + __ram_size__;

EXTERN(CONFIG);

SECTIONS
{
    .toboot :
    {
        KEEP(*(.rodata.CONFIG));
    } > FLASH

} INSERT AFTER .vector_table;

_stext = ADDR(.toboot) + SIZEOF(.toboot);

/* vim: set ft=ld : */
