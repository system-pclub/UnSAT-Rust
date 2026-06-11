#[doc = "Register `EPTOGGLE` reader"]
pub struct R(crate::pac::generic::R<EPTOGGLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::pac::generic::R<EPTOGGLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::pac::generic::R<EPTOGGLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::pac::generic::R<EPTOGGLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPTOGGLE` writer"]
pub struct W(crate::pac::generic::W<EPTOGGLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::pac::generic::W<EPTOGGLE_SPEC>;
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
impl From<crate::pac::generic::W<EPTOGGLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::pac::generic::W<EPTOGGLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOGGLE` reader - Endpoint data toggle: This field indicates the current value of the data toggle for the corresponding endpoint."]
pub struct TOGGLE_R(crate::pac::generic::FieldReader<u16, u16>);
impl TOGGLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TOGGLE_R(crate::pac::generic::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOGGLE_R {
    type Target = crate::pac::generic::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOGGLE` writer - Endpoint data toggle: This field indicates the current value of the data toggle for the corresponding endpoint."]
pub struct TOGGLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOGGLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Endpoint data toggle: This field indicates the current value of the data toggle for the corresponding endpoint."]
    #[inline(always)]
    pub fn toggle(&self) -> TOGGLE_R {
        TOGGLE_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Endpoint data toggle: This field indicates the current value of the data toggle for the corresponding endpoint."]
    #[inline(always)]
    pub fn toggle(&mut self) -> TOGGLE_W {
        TOGGLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Endpoint toggle register\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eptoggle](index.html) module"]
pub struct EPTOGGLE_SPEC;
impl crate::pac::generic::RegisterSpec for EPTOGGLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eptoggle::R](R) reader structure"]
impl crate::pac::generic::Readable for EPTOGGLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eptoggle::W](W) writer structure"]
impl crate::pac::generic::Writable for EPTOGGLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPTOGGLE to value 0"]
impl crate::pac::generic::Resettable for EPTOGGLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
