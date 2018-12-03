imtomu-rs
---

driver and hal crates targeted for [Tim's Open Micro USB](http://tomu.im/)

Includes support for tomu-bootloader config (toboot v2).

work in progress

- [X] toboot config
- [ ] timers
- [X] GPIO (most of the functionality is implemented)
- [ ] USB
- [ ] AES

toboot config
---

Application can interact with tomu bootloader by using `toboot_config` macro.
It's fully typesafe so there's no need to worry you're putting wrong config. It will even warns you if you're trying to lock bootloader entry like this:
```rust
toboot_config! {
    lock_entry: true,
}
```

![warns](https://f4.fudanchii.net/shx/putty_(3)_2018-12-02_04-08-41.png)

Full config as the following:
```rust
toboot_config! {
    config: [autorun_enable, irq_enable],
    lock_entry: false,
    // efm32hg309f64 has 64KiB flash memory,
    // each bit below represent 1 sector (1KiB)
    // which will be erased when tomu load its bootloader
    erase_mask_lo: 0, // 32bit uint
    erase_mask_hi: 0, // 32bit uint
}
```

Toboot api ref: [here](https://github.com/im-tomu/tomu-bootloader/blob/master/API.md).

examples
---
There are some examples on how to use tomu-hal in examples folder.
