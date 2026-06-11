#[doc = "Register `EPSKIP` reader"]
pub struct R(crate::pac::generic::R<EPSKIP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::pac::generic::R<EPSKIP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::pac::generic::R<EPSKIP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::pac::generic::R<EPSKIP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPSKIP` writer"]
pub struct W(crate::pac::generic::W<EPSKIP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::pac::generic::W<EPSKIP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::pac::generic::W<EPSKIP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::pac::generic::W<EPSKIP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SKIP` reader - Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software. When HW has deactivated the endpoint, it will clear this bit, but it will not modify the EPINUSE bit. An interrupt will be generated when the Active bit goes from 1 to 0. Note: In case of double-buffering, HW will only clear the Active bit of the buffer indicated by the EPINUSE bit."]
pub struct SKIP_R(crate::pac::generic::FieldReader<u16, u16>);
impl SKIP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SKIP_R(crate::pac::generic::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SKIP_R {
    type Target = crate::pac::generic::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SKIP` writer - Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software. When HW has deactivated the endpoint, it will clear this bit, but it will not modify the EPINUSE bit. An interrupt will be generated when the Active bit goes from 1 to 0. Note: In case of double-buffering, HW will only clear the Active bit of the buffer indicated by the EPINUSE bit."]
pub struct SKIP_W<'a> {
    w: &'a mut W,
}
impl<'a> SKIP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software. When HW has deactivated the endpoint, it will clear this bit, but it will not modify the EPINUSE bit. An interrupt will be generated when the Active bit goes from 1 to 0. Note: In case of double-buffering, HW will only clear the Active bit of the buffer indicated by the EPINUSE bit."]
    #[inline(always)]
    pub fn skip(&self) -> SKIP_R {
        SKIP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software. When HW has deactivated the endpoint, it will clear this bit, but it will not modify the EPINUSE bit. An interrupt will be generated when the Active bit goes from 1 to 0. Note: In case of double-buffering, HW will only clear the Active bit of the buffer indicated by the EPINUSE bit."]
    #[inline(always)]
    pub fn skip(&mut self) -> SKIP_W {
        SKIP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Endpoint skip\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epskip](index.html) module"]
pub struct EPSKIP_SPEC;
impl crate::pac::generic::RegisterSpec for EPSKIP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epskip::R](R) reader structure"]
impl crate::pac::generic::Readable for EPSKIP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epskip::W](W) writer structure"]
impl crate::pac::generic::Writable for EPSKIP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPSKIP to value 0"]
impl crate::pac::generic::Resettable for EPSKIP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
