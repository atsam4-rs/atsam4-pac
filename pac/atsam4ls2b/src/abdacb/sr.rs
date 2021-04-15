#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Transmit Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRDY_A {
    #[doc = "0: No Audio DAC TX Ready has occured since the last time ISR was read or since reset"]
    _0 = 0,
    #[doc = "1: At least one Audio DAC TX Ready has occured since the last time ISR was read or since reset"]
    _1 = 1,
}
impl From<TXRDY_A> for bool {
    #[inline(always)]
    fn from(variant: TXRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXRDY`"]
pub type TXRDY_R = crate::R<bool, TXRDY_A>;
impl TXRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRDY_A {
        match self.bits {
            false => TXRDY_A::_0,
            true => TXRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXRDY_A::_1
    }
}
#[doc = "Transmit Underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXUR_A {
    #[doc = "0: No Audio DAC Underrun has occured since the last time ISR was read or since reset"]
    _0 = 0,
    #[doc = "1: At least one Audio DAC Underrun has occured since the last time ISR was read or since reset"]
    _1 = 1,
}
impl From<TXUR_A> for bool {
    #[inline(always)]
    fn from(variant: TXUR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXUR`"]
pub type TXUR_R = crate::R<bool, TXUR_A>;
impl TXUR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXUR_A {
        match self.bits {
            false => TXUR_A::_0,
            true => TXUR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXUR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXUR_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - ABDACB Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit Underrun"]
    #[inline(always)]
    pub fn txur(&self) -> TXUR_R {
        TXUR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
