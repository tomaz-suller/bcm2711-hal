use super::dynpin::{DynPinId, DynPinMode};
use crate::pac;

#[allow(dead_code)]
enum FunctionSelect {
    Input,
    Output,
    AlternateFunction0,
    AlternateFunction1,
    AlternateFunction2,
    AlternateFunction3,
    AlternateFunction4,
    AlternateFunction5,
}

impl From<FunctionSelect> for u8 {
    fn from(value: FunctionSelect) -> Self {
        match value {
            FunctionSelect::Input => 0b000,
            FunctionSelect::Output => 0b001,
            FunctionSelect::AlternateFunction0 => 0b100,
            FunctionSelect::AlternateFunction1 => 0b101,
            FunctionSelect::AlternateFunction2 => 0b110,
            FunctionSelect::AlternateFunction3 => 0b111,
            FunctionSelect::AlternateFunction4 => 0b011,
            FunctionSelect::AlternateFunction5 => 0b010,
        }
    }
}

impl Default for FunctionSelect {
    fn default() -> Self {
        FunctionSelect::Input
    }
}

#[derive(Default)]
struct ModeFields {
    fsel: FunctionSelect,
}

impl From<DynPinMode> for ModeFields {
    fn from(value: DynPinMode) -> Self {
        let mut fields = Self::default();
        use DynPinMode::*;
        match value {
            Input(_) => fields.fsel = FunctionSelect::Input,
            Output(_) => fields.fsel = FunctionSelect::Output,
            Disabled(_) => todo!(),
            Function(_) => todo!(),
        };
        fields
    }
}

pub(super) unsafe trait RegisterInterface {
    fn id(&self) -> DynPinId;

    fn mask(&self) -> u32 {
        1 << (self.id().num % 32)
    }

    // TODO Output embedded_hal `PinSate`
    fn read_pin(&self) -> bool {
        let mask = self.mask();
        (match self.id().group() {
            0 => unsafe { &(*pac::GPIO::ptr()) }.gplev0.read().bits(),
            1 => unsafe { &(*pac::GPIO::ptr()) }.gplev1.read().bits(),
            _ => unreachable!(),
        }) & mask
            != 0
    }

    // TODO Receive embedded_hal `PinSate` as argument
    fn write_pin(&mut self, bit: bool) {
        let mask = self.mask();
        unsafe {
            match self.id().group() {
                0 => {
                    if bit {
                        (*pac::GPIO::ptr()).gpset0.write_with_zero(|w| w.bits(mask));
                    } else {
                        (*pac::GPIO::ptr()).gpclr0.write_with_zero(|w| w.bits(mask));
                    }
                }
                1 => {
                    if bit {
                        (*pac::GPIO::ptr()).gpset1.write_with_zero(|w| w.bits(mask));
                    } else {
                        (*pac::GPIO::ptr()).gpclr1.write_with_zero(|w| w.bits(mask));
                    }
                }
                _ => unreachable!(),
            }
        };
    }

    fn change_mode(&mut self, mode: DynPinMode) {
        let fields: ModeFields = mode.into();
        let fsel_offset = self.id().num % 10;
        let fsel = (u8::from(fields.fsel) << fsel_offset) as u32;
        unsafe {
            match self.id().fsel_group() {
                0 => (*pac::GPIO::ptr())
                    .gpfsel0
                    .write_with_zero(|w| w.bits(fsel)),
                1 => (*pac::GPIO::ptr())
                    .gpfsel1
                    .write_with_zero(|w| w.bits(fsel)),
                2 => (*pac::GPIO::ptr())
                    .gpfsel2
                    .write_with_zero(|w| w.bits(fsel)),
                3 => (*pac::GPIO::ptr())
                    .gpfsel3
                    .write_with_zero(|w| w.bits(fsel)),
                4 => (*pac::GPIO::ptr())
                    .gpfsel4
                    .write_with_zero(|w| w.bits(fsel)),
                5 => (*pac::GPIO::ptr())
                    .gpfsel5
                    .write_with_zero(|w| w.bits(fsel)),
                _ => unreachable!(),
            }
        }
    }
}

// #[inline]
// fn gpio_change_mode<M: PinMode>(num: usize, mode: M) {
//     let fields: ModeFields = mode.into();
//     (*pac::GPIO::ptr()).gpafen0
// }
