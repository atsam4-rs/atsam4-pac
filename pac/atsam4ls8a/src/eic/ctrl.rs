#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Reader of field `NMI`"]
pub type NMI_R = crate::R<bool, bool>;
#[doc = "Reader of field `INT1`"]
pub type INT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `INT2`"]
pub type INT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `INT3`"]
pub type INT3_R = crate::R<bool, bool>;
#[doc = "Reader of field `INT4`"]
pub type INT4_R = crate::R<bool, bool>;
#[doc = "Reader of field `INT5`"]
pub type INT5_R = crate::R<bool, bool>;
#[doc = "Reader of field `INT6`"]
pub type INT6_R = crate::R<bool, bool>;
#[doc = "Reader of field `INT7`"]
pub type INT7_R = crate::R<bool, bool>;
#[doc = "Reader of field `INT8`"]
pub type INT8_R = crate::R<bool, bool>;
#[doc = "Reader of field `INT9`"]
pub type INT9_R = crate::R<bool, bool>;
#[doc = "Reader of field `INT10`"]
pub type INT10_R = crate::R<bool, bool>;
#[doc = "Reader of field `INT11`"]
pub type INT11_R = crate::R<bool, bool>;
#[doc = "Reader of field `INT12`"]
pub type INT12_R = crate::R<bool, bool>;
#[doc = "Reader of field `INT13`"]
pub type INT13_R = crate::R<bool, bool>;
#[doc = "Reader of field `INT14`"]
pub type INT14_R = crate::R<bool, bool>;
#[doc = "Reader of field `INT15`"]
pub type INT15_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - External Non Maskable CPU interrupt"]
    #[inline(always)]
    pub fn nmi(&self) -> NMI_R {
        NMI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1"]
    #[inline(always)]
    pub fn int1(&self) -> INT1_R {
        INT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2"]
    #[inline(always)]
    pub fn int2(&self) -> INT2_R {
        INT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Interrupt 3"]
    #[inline(always)]
    pub fn int3(&self) -> INT3_R {
        INT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - External Interrupt 4"]
    #[inline(always)]
    pub fn int4(&self) -> INT4_R {
        INT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - External Interrupt 5"]
    #[inline(always)]
    pub fn int5(&self) -> INT5_R {
        INT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - External Interrupt 6"]
    #[inline(always)]
    pub fn int6(&self) -> INT6_R {
        INT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External Interrupt 7"]
    #[inline(always)]
    pub fn int7(&self) -> INT7_R {
        INT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - External Interrupt 8"]
    #[inline(always)]
    pub fn int8(&self) -> INT8_R {
        INT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - External Interrupt 9"]
    #[inline(always)]
    pub fn int9(&self) -> INT9_R {
        INT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - External Interrupt 10"]
    #[inline(always)]
    pub fn int10(&self) -> INT10_R {
        INT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - External Interrupt 11"]
    #[inline(always)]
    pub fn int11(&self) -> INT11_R {
        INT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - External Interrupt 12"]
    #[inline(always)]
    pub fn int12(&self) -> INT12_R {
        INT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - External Interrupt 13"]
    #[inline(always)]
    pub fn int13(&self) -> INT13_R {
        INT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - External Interrupt 14"]
    #[inline(always)]
    pub fn int14(&self) -> INT14_R {
        INT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - External Interrupt 15"]
    #[inline(always)]
    pub fn int15(&self) -> INT15_R {
        INT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
