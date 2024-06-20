# Rust driver for Analog Devices MAX1726x series of low-power fuel gauge ICs

This is a platform agnostic Rust driver for MAX1726x series of low-power fuel
gauge ICs, using the [`embedded-hal`] (v1) traits.

This driver allows you to:

- Read the IC version (see `get_version()`)

It supports:

- Blocking SPI using `embedded-hal 1.0`

## The devices

The following documents are referenced in this driver:

- [MAX1726x ModelGauge m5 EZ User Guide](https://www.analog.com/media/en/technical-documentation/user-guides/max1726x-modelgauge-m5-ez-user-guide.pdf)
- [ModelGauge m5 Host Side Software Implementation
  Guide](https://www.analog.com/media/en/technical-documentation/user-guides/modelgauge-m5-host-side-software-implementation-guide.pdf)
- [MAX17263 datasheet](https://www.analog.com/media/en/technical-documentation/data-sheets/MAX17263.pdf)

[](https://www.ti.com/lit/ds/symlink/tmp121.pdf)

## Usage

TODO
