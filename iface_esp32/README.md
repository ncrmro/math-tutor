# Local Development

- esp-rs: [Risc-V Targets Only](https://esp-rs.github.io/book/installation/riscv.html)
- [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) (used to install espflash pre built binary):
- [cargo-espflash](https://github.com/esp-rs/espflash/blob/main/espflash/README.md) (used to install espflash pre built binary):
- To access the serial port on ubuntu `sudo gpasswd --add ${USER} dialout`
- Remove this sudo chmod 666 /dev/ttyUSB0

```shell
cargo-espflash espflash flash --monitor
```

# Notes

https://wokwi.com/projects/362145427195567105