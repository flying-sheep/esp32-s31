ESP32-S31
=========

Supported since https://github.com/esp-rs/esp-idf/pull/608 and espflash 4.6.0

Flashing
--------

1. Connect USB-DBG port
2. `cargo run` runs it through `espflash flash --monitor`
3. We’re now in `esp_idf_monitor`: `Ctrl+T`→`Ctrl+H`: help, `Ctrl+T`→`Ctrl+R`: reset, `Ctrl+]`: quit.
