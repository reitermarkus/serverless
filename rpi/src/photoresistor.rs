use std::env;

use ads1x1x::{channel, interface::I2cInterface, ic::{Ads1115, Resolution16Bit}, mode, Ads1x1x, FullScaleRange::Within6_144V};
use embedded_hal::adc::OneShot;
use linux_embedded_hal::I2cdev;
use i2cdev::linux::LinuxI2CError;
use nb::block;

fn photoresistor() -> Result<Ads1x1x<I2cInterface<I2cdev>, Ads1115, Resolution16Bit, mode::OneShot>, ads1x1x::Error<LinuxI2CError>> {
  let dev = I2cdev::new(env::var("I2C_DEVICE").expect("I2C_DEVICE is not set")).map_err(|e| ads1x1x::Error::I2C(e))?;
  let mut adc = Ads1x1x::new_ads1115(dev, Default::default());
  adc.set_full_scale_range(Within6_144V)?;
  Ok(adc)
}

pub fn lux() -> Result<f64, ads1x1x::Error<LinuxI2CError>> {
  photoresistor().and_then(|mut photoresistor| {
    let value = block!(photoresistor.read(&mut channel::SingleA0)).unwrap();
    let adjusted_value = if value < 0 { 0.0 } else { f64::from(value) * 6.144 / f64::from(2i16.pow(15) - 1) };
    Ok(voltage_to_lux(adjusted_value))
  })
}

// These values are taken from the following example, they are not accurate for our specific resistor.
// https://www.allaboutcircuits.com/projects/design-a-luxmeter-using-a-light-dependent-resistor/
const LUX_CALC_SCALAR: u32 = 12518931;
const LUX_CALC_EXPONENT: f64 = -1.405;

fn voltage_to_lux(output_voltage: f64) -> f64 {
  let input_voltage = 5.0; // 5 v
  let resistance = 10000.0; // 10 kohm
  let resistance_ldr = (resistance * output_voltage) / (input_voltage - output_voltage);
  f64::from(LUX_CALC_SCALAR) * resistance_ldr.powf(LUX_CALC_EXPONENT)
}
