pub const TOBOOT_V2_MAGIC: u32 = 0x907070b2;
pub const TOBOOT_LOCK_ENTRY_MAGIC: u32 = 0x18349420;

/// Configuration for Tomu Bootloader (toboot)
///
/// Application can be configured to work with Tomu Bootloader,
/// and not all fields here should be set to work. Use `toboot_config!` macro
/// below to config only certain fields
#[repr(packed)]
pub struct TobootConfig {
    /// This is the magic header for toboot config,
    /// for toboot v2 the value should be: 0x907070b2
    pub magic: u32,

    /// Reserved value, no need to be set. Toboot will rewrite this with
    /// incrementally number and may be used to determined which image is the newest
    pub reserved_gen: u16,

    /// The start page of the program, there is no need to set this and should be left to
    /// its default value, for application this will be set to 16 (16 * 1024 = 0x4000).
    pub start: u8,

    /// Configuration bitmask, see `config_val` method below for possible values.
    pub config: u8,

    /// This can be set to disable manual bootloader entry. If it's set to 0x18349420
    /// User will not be able to enter bootloader via outer-pin shorting
    pub lock_entry: u32,

    /// Bitmasks for lower sectors that would need to be erased when updating program,
    /// each `1` indicating sectors that need to be erased (sector 1-31). Sectors used by toboot
    /// wont be erased.
    pub erase_mask_lo: u32,

    /// Bitmasks for higher sectors that would need to be erased when updating program,
    /// each `1` indicating sectors that need to be erased (sector 32-63).
    pub erase_mask_hi: u32,

    /// A hash for the entire header, minus this field. This is calculated automatically by toboot
    /// and don't need to be set.
    pub reserved_hash: u32,
}

#[cfg(not(feature = "custom-toboot-config"))]
#[used]
#[no_mangle]
pub static CONFIG: TobootConfig = TobootConfig {
    magic: TOBOOT_V2_MAGIC,
    reserved_gen: 0,
    start: 16,
    config: 0,
    lock_entry: 0,
    erase_mask_lo: 0,
    erase_mask_hi: 0,
    reserved_hash: 0,
};
