#[doc = "Reader of register TLV_DCO_30_TAG"]
pub type R = crate::R<u8, super::TLV_DCO_30_TAG>;
#[doc = "Writer for register TLV_DCO_30_TAG"]
pub type W = crate::W<u8, super::TLV_DCO_30_TAG>;
#[doc = "Register TLV_DCO_30_TAG `reset()`'s with value 0"]
impl crate::ResetValue for super::TLV_DCO_30_TAG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TLV_DCO_30_TAG`"]
pub type TLV_DCO_30_TAG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TLV_DCO_30_TAG`"]
pub struct TLV_DCO_30_TAG_W<'a> {
    w: &'a mut W,
}
impl<'a> TLV_DCO_30_TAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - TLV TAG_DCO30 TAG register"]
    #[inline(always)]
    pub fn tlv_dco_30_tag(&self) -> TLV_DCO_30_TAG_R {
        TLV_DCO_30_TAG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TLV TAG_DCO30 TAG register"]
    #[inline(always)]
    pub fn tlv_dco_30_tag(&mut self) -> TLV_DCO_30_TAG_W {
        TLV_DCO_30_TAG_W { w: self }
    }
}
