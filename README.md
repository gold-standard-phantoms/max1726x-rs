# Rust driver for Analog Devices MAX1726x series of low-power fuel gauge ICs

This is a platform agnostic Rust driver for MAX1726x series of low-power fuel
gauge ICs, using the [`embedded-hal`] (v1) traits.

This driver allows you to:

- Read battery voltage, current, temperature, and state of charge
- Configure and manage hibernate mode settings
- Control LED indicators (MAX17263)
- Configure and monitor battery charge status
- Perform EZ configuration setup
- Monitor various status flags and alerts
- Access and control power management features

It supports:

- Blocking I2C using `embedded-hal 1.0`
- No-std environments
- Comprehensive register access and configuration
- Type-safe register bit field manipulation

## The devices

This driver is compatible with the MAX1726x series of fuel gauge ICs, specifically tested with the MAX17263. The following documents are referenced in this driver:

- [MAX1726x ModelGauge m5 EZ User Guide](https://www.analog.com/media/en/technical-documentation/user-guides/max1726x-modelgauge-m5-ez-user-guide.pdf)
- [ModelGauge m5 Host Side Software Implementation Guide](https://www.analog.com/media/en/technical-documentation/user-guides/modelgauge-m5-host-side-software-implementation-guide.pdf)
- [MAX17263 datasheet](https://www.analog.com/media/en/technical-documentation/data-sheets/MAX17263.pdf)

## Usage

### Basic Setup

```rust
use max1726x::{Max1726x, max17263::registers::Max17263RegisterResolver};
// Create a new instance with a sense resistor value of 0.01 ohms
let register_resolver = Max17263RegisterResolver::new(0.01);
let mut fuel_gauge = Max1726x::new(i2c, register_resolver);
// Read battery measurements
let voltage = fuel_gauge.battery_voltage()?;
let current = fuel_gauge.battery_current()?;
let temperature = fuel_gauge.battery_temperature()?;
let soc = fuel_gauge.battery_state_of_charge()?;
```

### EZ Configuration

```rust
use max1726x::{EzConfig, registers::VEmpty};
let ez_config = EzConfig {
charge_voltage_mv: 4200,
design_cap_mah: 2500,
i_chg_term_ma: 100,
v_empty_mv: VEmpty::init(3300, 3880), // Empty and recovery voltages
};
fuel_gauge.ez_config(delay, ez_config)?;
```

### LED Control (MAX17263)

```rust
use max1726x::max17263::registers::{LedCfg1, LedCfg2, LedCfg3};
// Configure LED settings
let led_cfg1 = LedCfg1::default()
.with_n_bars(5)
.with_gr_en(true)
.with_led_md(1);
fuel_gauge.write_bitfield_to_register(led_cfg1)?;
```

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.