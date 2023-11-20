# Overview

A cheap, single-purpose educational device, akin to a Tamagotchi, offers significant benefits for impoverished communities. Its low cost and focused functionality provide targeted and engaging learning experiences without the need for advanced infrastructure like internet or electricity. These devices are easy to distribute, robust, and require minimal maintenance, making them particularly suitable for remote and under-resourced areas.

- iface_esp32: ESP32-C3 Risc processor, two GPIO buttons and display
- iface_terminal_cli: Terminal CLI based math learninig tutor
- math_tutor_core: Shared library designed so it can be used on embedded devices (ESP32-C3)


Cargo workspace has trouble right now because of the custom ESP32-C3 Risc toolchain.


```shell
cargo build --workspace  
```

The terminal CLI can be accessed by running the following from the `iface_terminal_cli`

```shell
cargo run
```
