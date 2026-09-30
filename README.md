ESP32-S31
=========

Supppoered since https://github.com/esp-rs/esp-idf/pull/608

Flashing
--------

1. Connect USB-DBG port
2. Run `use ./flash.nu; flash` (Once it supports the ESP32-S31, switch back to `espflash flash --monitor`)
3. We’re now in `esp_idf_monitor`: `Ctrl+T`→`Ctrl+H`: help, `Ctrl+T`→`Ctrl+R`: reset, `Ctrl+]`: quit.
