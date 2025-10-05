#![cfg_attr(feature = "embedded", no_std)]

use defmt::info;

#[cfg(not(test))]
use embassy_stm32::adc::{Adc, Instance};

pub trait AnalogRead<Pin> {
    type Value;

    fn read(&mut self, pin: &mut Pin) -> Self::Value;
}

#[cfg(feature = "embedded")]
impl<Pin, T> AnalogRead<Pin> for Adc<'_, T>
where
    Pin: embassy_stm32::adc::AdcChannel<T>,
    T: Instance,
{
    type Value = u16;

    fn read(&mut self, pin: &mut Pin) -> Self::Value {
        self.blocking_read(pin)
    }
}

pub fn is_position_max<Channel, Pin>(adc: &mut Channel, pin: &mut Pin) -> bool
where
    Channel: AnalogRead<Pin, Value = u16>,
{
    let value = adc.read(pin);
    #[cfg(feature = "embedded")]
    info!("ADC reading: {}", value);
    value > 2000
}

#[cfg(test)]
mod tests {
    use crate::{is_position_max, AnalogRead};
    use rstest::rstest;

    struct PinMock(u16);

    struct AdcMock;

    impl AdcMock {
        pub fn new() -> Self {
            Self
        }
    }

    impl AnalogRead<PinMock> for AdcMock {
        type Value = u16;

        fn read(&mut self, pin: &mut PinMock) -> Self::Value {
            pin.0
        }
    }

    #[rstest]
    #[case::below_threshold(1999, false)]
    #[case::at_threshold(2000, false)]
    #[case::above_threshold(2001, true)]
    fn test_boundary_conditions(#[case] input: u16, #[case] expected: bool) {
        // Given
        let mut pin_mock = PinMock(input);
        let mut adc_mock = AdcMock::new();

        // When
        let value = is_position_max(&mut adc_mock, &mut pin_mock);

        // Then
        assert_eq!(value, expected);
    }
}
