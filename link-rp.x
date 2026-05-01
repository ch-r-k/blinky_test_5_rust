/* Place RP2040 second-stage bootloader at flash base (0x10000000). */
SECTIONS
{
    .boot2 ORIGIN(BOOT2) :
    {
        KEEP(*(.boot2));
    } > BOOT2
} INSERT BEFORE .vector_table;
