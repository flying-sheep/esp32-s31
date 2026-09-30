#!/usr/bin/env nu
# Cargo runner: flash the ELF and open a serial monitor.
# TODO: switch back to `espflash flash --monitor` once it has esp32s31 support.

export def main [elf: path = 'target/riscv32imafc-esp-espidf/debug/esp32-s31'] {
    let dir = $elf | path dirname
    const py = path self . | path join .embuild/espressif/python_env/idf6.1_py3.14_env/bin/python
    let port = $env.ESPPORT? | default /dev/ttyACM0

    ^$py -m esptool --chip esp32s31 elf2image -o $"($elf).bin" $elf
    (^$py -m esptool --chip esp32s31 -p $port write-flash
        0x2000 ($dir | path join bootloader.bin)
        0x8000 ($dir | path join partition-table.bin)
        0x10000 $"($elf).bin")
    exec $py -m esp_idf_monitor -p $port $elf
}
