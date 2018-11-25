MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x00004000, LENGTH = 0xC000
  RAM : ORIGIN = 0x20000000 + 8, LENGTH = 0x2000 - 8
}

__app_start__ = ORIGIN(FLASH);
__app_end__   = __app_start__ + LENGTH(FLASH);
__ram_start__ = ORIGIN(RAM);
__ram_size__  = LENGTH(RAM);
__ram_end__   = __ram_start__ + __ram_size__;
boot_token    = ORIGIN(RAM) - 8;

SECTIONS
{
    .toboot :
    {
        KEEP(*(.toboot.config));
    } > FLASH

} INSERT AFTER .vector_table;

_stext = ORIGIN(FLASH) + 0x94 + SIZEOF(.toboot);

/* vim: set ft=ld : */
