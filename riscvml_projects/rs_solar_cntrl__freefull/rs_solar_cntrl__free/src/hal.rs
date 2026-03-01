//! Hardware Abstraction Layer — peripheral initialization.
//!
//! `SolarPeripherals` consumes the raw ESP32-P4 peripherals and produces
//! initialized, typed handles for each subsystem. This is the single place
//! where pin assignments (from `config`) are wired to actual hardware.

use crate::config;
use crate::ffi::{self, I2cBus, PwmChannel};
use crate::types::SolarError;

/// All initialized peripherals needed by the solar controller.
pub struct SolarPeripherals {
    /// I2C bus for inclinometer communication
    pub i2c: I2cBus,
    /// ADC channel numbers for string voltage measurements [A, B, C]
    pub adc_voltage_channels: [u8; 3],
    /// ADC channel numbers for string current measurements [A, B, C]
    pub adc_current_channels: [u8; 3],
    /// PWM channel for actuator speed
    pub pwm: PwmChannel,
    /// H-bridge direction pin A
    pub hbridge_dir_a: u8,
    /// H-bridge direction pin B
    pub hbridge_dir_b: u8,
    /// H-bridge enable pin
    pub hbridge_en: u8,
    /// Wind limit switch input pin
    pub wind_switch_pin: u8,
    /// Status LED pin
    pub status_led_pin: u8,
}

impl SolarPeripherals {
    /// Initialize all peripherals from config pin assignments.
    ///
    /// This function should be called once at startup. It configures:
    /// - I2C master bus (SDA/SCL pins, 400kHz)
    /// - ADC1 channels for 3x voltage + 3x current
    /// - LEDC PWM channel for actuator motor
    /// - GPIO outputs for H-bridge control + status LED
    /// - GPIO input with pull-up for wind limit switch
    pub fn init() -> Result<Self, SolarError> {
        // Initialize I2C bus for inclinometer
        let i2c = ffi::i2c_init(
            config::PIN_I2C_SDA,
            config::PIN_I2C_SCL,
            config::I2C_CLOCK_HZ,
        )?;

        // Initialize PWM for actuator (1kHz carrier frequency)
        let pwm = ffi::pwm_init(config::PIN_ACTUATOR_PWM, 1000)?;

        // Configure H-bridge GPIO outputs (start disabled)
        ffi::gpio_set(config::PIN_HBRIDGE_EN, false)?;
        ffi::gpio_set(config::PIN_HBRIDGE_DIR_A, false)?;
        ffi::gpio_set(config::PIN_HBRIDGE_DIR_B, false)?;

        // Configure status LED (start off)
        ffi::gpio_set(config::PIN_STATUS_LED, false)?;

        Ok(SolarPeripherals {
            i2c,
            adc_voltage_channels: config::PIN_ADC_VOLTAGE,
            adc_current_channels: config::PIN_ADC_CURRENT,
            pwm,
            hbridge_dir_a: config::PIN_HBRIDGE_DIR_A,
            hbridge_dir_b: config::PIN_HBRIDGE_DIR_B,
            hbridge_en: config::PIN_HBRIDGE_EN,
            wind_switch_pin: config::PIN_WIND_SWITCH,
            status_led_pin: config::PIN_STATUS_LED,
        })
    }
}
