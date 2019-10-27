Tomu board boot sequence
---

Tomu board officially flashed with toboot as its bootloader.

The boot sequence is as following:

1. `reset_handler` vector  
   The interrupt vector table has this entry set to a function called
   `Reset_Handler` which will be called each time the mcu got `reset` interrupt request.
   `Reset_Handler` then will run this function in sequence:
   `init_crt()` -> `__early_init()` -> `bootloader_main()`

1. `init_crt`  
   Toboot first will relocate the entire `.data` and `.text` session from flash to RAM.
   After that the `.bss` section will be reset to 0.
   At this point our vector table was read from flash at offset 0, toboot then also copying
   this section to RAM, then setting `SCB`'s `VTOR` register to the vector address in the RAM.

1. `__early_init`  
   In this function toboot enable some of the peripherals required at boot along with their clocks.
   
   Toboot first enable `HFPERCLK` (should be automatically up at reset though), then enable clocks for `GPIO`, `USART`, and `TIMER`. Then goes enable `LE` low energy devices clock, `DMA`, `USB`, and `USBC` clocks from `HFCORECLK`. Both `HFCORECLK` and `HFPERCLK` are sourced from `HFRCO` at 14MHz now.

   clock state:
   - `HFCLK` source: `HFRCO` 14Mhz no clock division
   - `HFCORECLK` enabled, no clock division
   - `HFPERCLK` enabled, no clock division
   - `HFCORECLK` enables: `LE`, `DMA`, `USB`, `USBC`
   - `HFPERCLK` enables: `GPIO`, `USART`, `TIMER`

   This function then activate low frequency clocks, `LFCLK`, by selecting `LFRCO` for `LFA`, and `HFCORECLKDIV2` (`HFCORECLK` divided by 2) for `LFB`. It then enable `LFC` clock from `LFRCO` in separate assignment with bitmask to keep `LFA` and `LFB` configuration intact, and enable clock for `USBLE` (clocked by `LFC`).

   clock state:
   - `LFACLK` source: `LFRCO` 32KHz
   - `LFBCLK` source: `HFCORECLKLEDIV2` (`HFCORECLK` / 2)
   - `LFCCLK` source: `LFRCO` 32KHz
   - `LFCCLKEN0` enable `USBLE` bit.
   - All `LFRCO` sourced clocks above is not active yet as we haven't started `LFRCO` here

   Toboot then configure `USHFRCO` to operate in 48MHz band, then enable USB clock recovery by setting `EN` bit for `USBCRCTRL` register.

   After that, toboot goes setting the USB system so it can go into low energy mode when idle by configuring `USB`'s `CTRL` register with `LEMIDLEEN` bit set, it also tell USB PHY control to go into energy saving state when USB system go into low energy mode by setting `USB`'s `CTRL` register with `LEMPHYCTRL` bit set.

   Toboot then setting `HFRCO` frequency to 21MHz by setting `HFRCOCTRL`'s `BAND` register to 4 (i.e. 4 << 8) and write `HFRCO` clock calibration value for 21MHz from `DEVINFO`. 

   By default `HFRCO` is already enabled in 14 MHz clock speed, and at this point toboot explicitly setting efm32hg's oscillator register, `OSCENCMD`, to enable `HFRCO` again, now with clock frequency 21MHz.

   Then toboot reconfigure `HFCLK` to select `HFRCO` as source clock, by setting `CMU`'s `CMD` with `HFCLKSEL_HFRCO`.

   clock state:
   - `USHFRCO` 48MHz
   - `USB->CTRL` = `LEMIDLEEN | LEMPHYCTRL`
   - `HFRCOCTRL` = `(4 << 8) | DEVINFO->HFRCOCAL1` set `HFRCO` to 21MHz
   - `HFCLK` reconfigured to use `HFRCO` at 21MHz (previously it's already set to use `HFRCO` at 14MHz)

   After this, toboot then setting up the 2 available leds. It first set GPIO at port.0 pin.0 (PA0) to normal wired-and mode (open drain), and drive it high to turn off the green led.

   Then it set GPIO at port.1 pin.7 (PB7) also to normal wired-and mode (open drain), and drive it low to turn on the red led.

   Toboot then enables watchdog.

   It re-assign watchdog's clock source with `ULFRCO`, re-enable watchdog, and set the `PERSEL` register bit to 3. This is setting the watchdog to be trigger reset after 65 clock cycle. In toboot case, it's 65 / 1KHz so it will reset after 0.065s.

   Then toboot starts RTC clock by calling `start_rtc` function.

   `start_rtc`  
   ---
   Toboot first enable `LFRCO` oscillator, then enable `RTC` to be clocked by `LFA`. This function then goes clearing all interrupt flag for `RTC`, `COMP0`, `COMP1`, and `OF` (overflow). Then toboot set `COMP0` at (250 * 32768) / 1000 which translate to 250ms period. Toboot then enable interrupt at `COMP0` coupled with `COMP0` value before, this will causing `COMP0` interrupt every 250ms. But RTC hasn't running yet, toboot continue enabling `NVIC` interrupt service on `RTC` by setting `NVIC`'s `ISER` register and setting `RTC`'s IRQ bit.

   Lastly, this function set `RTC` to tick until `COMP0` value, keep running at debug (`DEBUGRUN` bit set), and enable the `RTC` to run.

   clock state:
   - `LFRCO` started here, running at 32KHz (32768Hz)
   - `COMP0` set to (250 * 32768) / 1000
   - `RTC` set to tick until `COMP0` value (250ms).