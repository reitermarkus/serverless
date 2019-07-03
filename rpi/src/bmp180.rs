pub const BMP180_ADDRESS: u16 = 0x77;

use std::env;

use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};
use i2cdev_bmp180::{BMP180BarometerThermometer, BMP180PressureMode::BMP180Standard};
use i2csensors::Thermometer;

fn barometer() -> Result<BMP180BarometerThermometer<LinuxI2CDevice>, LinuxI2CError> {
  let dev = LinuxI2CDevice::new(env::var("I2C_DEVICE").expect("I2C_DEVICE is not set"), BMP180_ADDRESS)?;
  BMP180BarometerThermometer::new(dev, BMP180Standard)
}

pub fn pressure() -> Result<f32, LinuxI2CError> {
  barometer().and_then(|mut barometer| barometer.pressure_hpa())
}

pub fn temperature() -> Result<f32, LinuxI2CError> {
  barometer().and_then(|mut barometer| barometer.temperature_celsius())
}
