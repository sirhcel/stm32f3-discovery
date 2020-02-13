pub mod interrupt;

use switch_hal::{ActiveHigh, InputSwitch, Switch};
use stm32f3xx_hal::gpio::gpioa::PA0;
use stm32f3xx_hal::gpio::{Floating, Input};

/// Wrapper struct around `ActiveHighButton<PA0<Input<Floating>>>`
/// It's floating because there's an external pull down resistor and low pass filter circuit.
pub struct UserButton(Switch<PA0<Input<Floating>>, ActiveHigh>);

impl UserButton {
    /// Typesafe constructor for the UserButton peripheral on PA0.
    /// It's impossible to construct this button with the wrong pin or pin state.
    /// It's also impossible to construct more than one `UserButton` instance because `gpioa.pa0` is moved upon construction.
    pub fn new(pa0: PA0<Input<Floating>>) -> Self {
        UserButton(Switch::<_, ActiveHigh>::new(pa0))
    }
}

impl InputSwitch for UserButton {
    type Error = ();
    fn is_active(&self) -> Result<bool, Self::Error> {
        self.0.is_active()
    }
}