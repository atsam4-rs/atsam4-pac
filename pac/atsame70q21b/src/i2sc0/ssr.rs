#[doc = "Writer for register SSR"]
pub type W = crate::W<u32, super::SSR>;
#[doc = "Register SSR `reset()`'s with value 0"]
impl crate::ResetValue for super::SSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RXOR`"]
pub struct RXOR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Write proxy for field `TXUR`"]
pub struct TXUR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Write proxy for field `RXORCH`"]
pub struct RXORCH_W<'a> {
    w: &'a mut W,
}
impl<'a> RXORCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `TXURCH`"]
pub struct TXURCH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXURCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
impl W {
    #[doc = "Bit 2 - Receive Overrun Status Set"]
    #[inline(always)]
    pub fn rxor(&mut self) -> RXOR_W {
        RXOR_W { w: self }
    }
    #[doc = "Bit 6 - Transmit Underrun Status Set"]
    #[inline(always)]
    pub fn txur(&mut self) -> TXUR_W {
        TXUR_W { w: self }
    }
    #[doc = "Bits 8:9 - Receive Overrun Per Channel Status Set"]
    #[inline(always)]
    pub fn rxorch(&mut self) -> RXORCH_W {
        RXORCH_W { w: self }
    }
    #[doc = "Bits 20:21 - Transmit Underrun Per Channel Status Set"]
    #[inline(always)]
    pub fn txurch(&mut self) -> TXURCH_W {
        TXURCH_W { w: self }
    }
}
