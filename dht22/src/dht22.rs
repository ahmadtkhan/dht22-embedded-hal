//! DHT22 integrated for RP2040
//!

use embassy_time::{Duration, Instant, Timer};
use embassy_rp::gpio::{Flex, Level, Output, Pin};
use panic_probe as _;
use libm::{powf, roundf};

use crate::error::SensorError;

#[derive(Debug)]
pub struct Reading<T, U> {
    pub temp: T, 
    pub hum: U,
}

pub trait F32Utils {
    fn round_fixed(&self, digits: u8) -> f32;
}

impl F32Utils for f32 {
    fn round_fixed(&self, digits: u8) -> f32 {
        let factor = powf(10.0, digits as f32);
        roundf(self * factor) / factor
    }
}

pub struct DHT22<'a>{
    pin: Flex<'a>,
}

impl<'a> DHT22<'a> {
    pub fn new(pin: Flex<'a>) ->Self {
        Self { pin }
    }
    pub async fn read(&mut self) -> Result<Reading<f32, f32>, SensorError>{
        self.pin.set_as_output();
        self.pin.set_low();
        Timer::after(Duration::from_micros(1000)).await;

        self.pin.set_as_input();

        self.wait_for_high(Duration::from_millis(5)).await?;
        self.wait_for_low(Duration::from_millis(5)).await?;

        self.wait_for_high(Duration::from_millis(5)).await?;
        self.wait_for_low(Duration::from_millis(5)).await?;

        let mut data = [0u8; 4];
        for i in 0..4 {
            data[i] = self.read_byte().await?;
        }
        let checksum = self.read_byte().await?;

        let sum = data.iter().fold(0u8, |acc, &v| acc.wrapping_add(v));
        if sum != checksum {
            return Err(SensorError::ChecksumMismatch);
        }
        let raw_hum = ((data[0] as u16) << 8) | data[1] as u16;
        let hum = (raw_hum as f32) * 0.1;

        let raw_temp = ((data[2] as u16) << 8) | data[3] as u16;
        let is_neg = (raw_temp & 0x8000) != 0;
        let abs_temp = (raw_temp & 0x7fff) as f32 * 0.1;
        let temp = if is_neg {-abs_temp} else {abs_temp};

        Ok (Reading {
            temp : temp.round_fixed(2),
            hum: hum.round_fixed(2),
        })
    }
    
    async fn yield_now() {
        Timer::after(Duration::from_micros(1)).await;
    }

    async fn read_byte(&mut self) -> Result<u8, SensorError> {
        let mut val = 0u8;
        for bit_i in 0..8{
            self.wait_for_high(Duration::from_micros(200)).await?;

            let t_high = self.measure_high_time(Duration::from_micros(200)).await?;

            if t_high > 35 {
                val |= 1 << (7 - bit_i);
            }
        }
        Ok(val)
    }
    
    async fn wait_for_high(&mut self, timeout: Duration) -> Result<(), SensorError>  {
        let start = Instant::now();
        while !self.pin.is_high(){
            if Instant::now() - start > timeout {
                return Err(SensorError::TimeoutWaitingForHigh);
            }
            Self::yield_now().await;
        }
        Ok(())
    }

    async fn wait_for_low(&mut self, timeout: Duration) -> Result<(), SensorError> {
        let start = Instant::now();
        while !self.pin.is_low(){
            if Instant::now() - start > timeout {
                return Err(SensorError::TimeoutWaitingForLow);
            }
            Self::yield_now().await;
        }
        Ok(())
    }

    async fn measure_high_time(&mut self, max_wait: Duration) -> Result<u64, SensorError> {
        let start = Instant::now();
        while !self.pin.is_high(){
            let elapsed = Instant::now() - start;
            if elapsed > max_wait {
                return Err(SensorError::TimeoutGeneric);
            }
            Self::yield_now().await;
        }
        let micros = (Instant::now() - start).as_micros();
        Ok(micros)
    }

}

