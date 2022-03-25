#[doc = "Reader of register PH_MODEH"]
pub type R = crate::R<u32, super::PH_MODEH>;
#[doc = "Writer for register PH_MODEH"]
pub type W = crate::W<u32, super::PH_MODEH>;
#[doc = "Register PH_MODEH `reset()`'s with value 0"]
impl crate::ResetValue for super::PH_MODEH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pin 8 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE8_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output"]
    PUSHPULL = 4,
    #[doc = "5: Push-pull using alternate control"]
    PUSHPULLALT = 5,
    #[doc = "6: Wired-or output"]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output"]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output using alternate control"]
    WIREDANDALT = 12,
    #[doc = "13: Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER = 13,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER = 15,
}
impl From<MODE8_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE8_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE8`"]
pub type MODE8_R = crate::R<u8, MODE8_A>;
impl MODE8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE8_A {
        match self.bits {
            0 => MODE8_A::DISABLED,
            1 => MODE8_A::INPUT,
            2 => MODE8_A::INPUTPULL,
            3 => MODE8_A::INPUTPULLFILTER,
            4 => MODE8_A::PUSHPULL,
            5 => MODE8_A::PUSHPULLALT,
            6 => MODE8_A::WIREDOR,
            7 => MODE8_A::WIREDORPULLDOWN,
            8 => MODE8_A::WIREDAND,
            9 => MODE8_A::WIREDANDFILTER,
            10 => MODE8_A::WIREDANDPULLUP,
            11 => MODE8_A::WIREDANDPULLUPFILTER,
            12 => MODE8_A::WIREDANDALT,
            13 => MODE8_A::WIREDANDALTFILTER,
            14 => MODE8_A::WIREDANDALTPULLUP,
            15 => MODE8_A::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE8_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE8_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE8_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE8_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE8_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE8_A::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE8_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE8_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE8_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE8_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE8_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE8_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE8_A::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE8_A::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE8_A::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE8_A::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Write proxy for field `MODE8`"]
pub struct MODE8_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE8_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE8_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE8_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE8_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE8_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE8_A::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE8_A::PUSHPULLALT)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE8_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE8_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE8_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE8_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE8_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE8_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE8_A::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE8_A::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE8_A::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE8_A::WIREDANDALTPULLUPFILTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Pin 9 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE9_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output"]
    PUSHPULL = 4,
    #[doc = "5: Push-pull using alternate control"]
    PUSHPULLALT = 5,
    #[doc = "6: Wired-or output"]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output"]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output using alternate control"]
    WIREDANDALT = 12,
    #[doc = "13: Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER = 13,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER = 15,
}
impl From<MODE9_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE9_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE9`"]
pub type MODE9_R = crate::R<u8, MODE9_A>;
impl MODE9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE9_A {
        match self.bits {
            0 => MODE9_A::DISABLED,
            1 => MODE9_A::INPUT,
            2 => MODE9_A::INPUTPULL,
            3 => MODE9_A::INPUTPULLFILTER,
            4 => MODE9_A::PUSHPULL,
            5 => MODE9_A::PUSHPULLALT,
            6 => MODE9_A::WIREDOR,
            7 => MODE9_A::WIREDORPULLDOWN,
            8 => MODE9_A::WIREDAND,
            9 => MODE9_A::WIREDANDFILTER,
            10 => MODE9_A::WIREDANDPULLUP,
            11 => MODE9_A::WIREDANDPULLUPFILTER,
            12 => MODE9_A::WIREDANDALT,
            13 => MODE9_A::WIREDANDALTFILTER,
            14 => MODE9_A::WIREDANDALTPULLUP,
            15 => MODE9_A::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE9_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE9_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE9_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE9_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE9_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE9_A::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE9_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE9_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE9_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE9_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE9_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE9_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE9_A::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE9_A::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE9_A::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE9_A::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Write proxy for field `MODE9`"]
pub struct MODE9_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE9_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE9_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE9_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE9_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE9_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE9_A::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE9_A::PUSHPULLALT)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE9_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE9_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE9_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE9_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE9_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE9_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE9_A::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE9_A::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE9_A::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE9_A::WIREDANDALTPULLUPFILTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Pin 10 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE10_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output"]
    PUSHPULL = 4,
    #[doc = "5: Push-pull using alternate control"]
    PUSHPULLALT = 5,
    #[doc = "6: Wired-or output"]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output"]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output using alternate control"]
    WIREDANDALT = 12,
    #[doc = "13: Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER = 13,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER = 15,
}
impl From<MODE10_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE10_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE10`"]
pub type MODE10_R = crate::R<u8, MODE10_A>;
impl MODE10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE10_A {
        match self.bits {
            0 => MODE10_A::DISABLED,
            1 => MODE10_A::INPUT,
            2 => MODE10_A::INPUTPULL,
            3 => MODE10_A::INPUTPULLFILTER,
            4 => MODE10_A::PUSHPULL,
            5 => MODE10_A::PUSHPULLALT,
            6 => MODE10_A::WIREDOR,
            7 => MODE10_A::WIREDORPULLDOWN,
            8 => MODE10_A::WIREDAND,
            9 => MODE10_A::WIREDANDFILTER,
            10 => MODE10_A::WIREDANDPULLUP,
            11 => MODE10_A::WIREDANDPULLUPFILTER,
            12 => MODE10_A::WIREDANDALT,
            13 => MODE10_A::WIREDANDALTFILTER,
            14 => MODE10_A::WIREDANDALTPULLUP,
            15 => MODE10_A::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE10_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE10_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE10_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE10_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE10_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE10_A::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE10_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE10_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE10_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE10_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE10_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE10_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE10_A::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE10_A::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE10_A::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE10_A::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Write proxy for field `MODE10`"]
pub struct MODE10_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE10_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE10_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE10_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE10_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE10_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE10_A::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE10_A::PUSHPULLALT)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE10_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE10_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE10_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE10_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE10_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE10_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE10_A::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE10_A::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE10_A::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE10_A::WIREDANDALTPULLUPFILTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Pin 11 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE11_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output"]
    PUSHPULL = 4,
    #[doc = "5: Push-pull using alternate control"]
    PUSHPULLALT = 5,
    #[doc = "6: Wired-or output"]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output"]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output using alternate control"]
    WIREDANDALT = 12,
    #[doc = "13: Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER = 13,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER = 15,
}
impl From<MODE11_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE11_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE11`"]
pub type MODE11_R = crate::R<u8, MODE11_A>;
impl MODE11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE11_A {
        match self.bits {
            0 => MODE11_A::DISABLED,
            1 => MODE11_A::INPUT,
            2 => MODE11_A::INPUTPULL,
            3 => MODE11_A::INPUTPULLFILTER,
            4 => MODE11_A::PUSHPULL,
            5 => MODE11_A::PUSHPULLALT,
            6 => MODE11_A::WIREDOR,
            7 => MODE11_A::WIREDORPULLDOWN,
            8 => MODE11_A::WIREDAND,
            9 => MODE11_A::WIREDANDFILTER,
            10 => MODE11_A::WIREDANDPULLUP,
            11 => MODE11_A::WIREDANDPULLUPFILTER,
            12 => MODE11_A::WIREDANDALT,
            13 => MODE11_A::WIREDANDALTFILTER,
            14 => MODE11_A::WIREDANDALTPULLUP,
            15 => MODE11_A::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE11_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE11_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE11_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE11_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE11_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE11_A::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE11_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE11_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE11_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE11_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE11_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE11_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE11_A::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE11_A::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE11_A::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE11_A::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Write proxy for field `MODE11`"]
pub struct MODE11_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE11_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE11_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE11_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE11_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE11_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE11_A::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE11_A::PUSHPULLALT)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE11_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE11_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE11_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE11_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE11_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE11_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE11_A::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE11_A::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE11_A::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE11_A::WIREDANDALTPULLUPFILTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Pin 12 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE12_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output"]
    PUSHPULL = 4,
    #[doc = "5: Push-pull using alternate control"]
    PUSHPULLALT = 5,
    #[doc = "6: Wired-or output"]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output"]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output using alternate control"]
    WIREDANDALT = 12,
    #[doc = "13: Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER = 13,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER = 15,
}
impl From<MODE12_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE12_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE12`"]
pub type MODE12_R = crate::R<u8, MODE12_A>;
impl MODE12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE12_A {
        match self.bits {
            0 => MODE12_A::DISABLED,
            1 => MODE12_A::INPUT,
            2 => MODE12_A::INPUTPULL,
            3 => MODE12_A::INPUTPULLFILTER,
            4 => MODE12_A::PUSHPULL,
            5 => MODE12_A::PUSHPULLALT,
            6 => MODE12_A::WIREDOR,
            7 => MODE12_A::WIREDORPULLDOWN,
            8 => MODE12_A::WIREDAND,
            9 => MODE12_A::WIREDANDFILTER,
            10 => MODE12_A::WIREDANDPULLUP,
            11 => MODE12_A::WIREDANDPULLUPFILTER,
            12 => MODE12_A::WIREDANDALT,
            13 => MODE12_A::WIREDANDALTFILTER,
            14 => MODE12_A::WIREDANDALTPULLUP,
            15 => MODE12_A::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE12_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE12_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE12_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE12_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE12_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE12_A::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE12_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE12_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE12_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE12_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE12_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE12_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE12_A::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE12_A::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE12_A::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE12_A::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Write proxy for field `MODE12`"]
pub struct MODE12_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE12_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE12_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE12_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE12_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE12_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE12_A::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE12_A::PUSHPULLALT)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE12_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE12_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE12_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE12_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE12_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE12_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE12_A::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE12_A::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE12_A::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE12_A::WIREDANDALTPULLUPFILTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Pin 13 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE13_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output"]
    PUSHPULL = 4,
    #[doc = "5: Push-pull using alternate control"]
    PUSHPULLALT = 5,
    #[doc = "6: Wired-or output"]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output"]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output using alternate control"]
    WIREDANDALT = 12,
    #[doc = "13: Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER = 13,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER = 15,
}
impl From<MODE13_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE13_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE13`"]
pub type MODE13_R = crate::R<u8, MODE13_A>;
impl MODE13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE13_A {
        match self.bits {
            0 => MODE13_A::DISABLED,
            1 => MODE13_A::INPUT,
            2 => MODE13_A::INPUTPULL,
            3 => MODE13_A::INPUTPULLFILTER,
            4 => MODE13_A::PUSHPULL,
            5 => MODE13_A::PUSHPULLALT,
            6 => MODE13_A::WIREDOR,
            7 => MODE13_A::WIREDORPULLDOWN,
            8 => MODE13_A::WIREDAND,
            9 => MODE13_A::WIREDANDFILTER,
            10 => MODE13_A::WIREDANDPULLUP,
            11 => MODE13_A::WIREDANDPULLUPFILTER,
            12 => MODE13_A::WIREDANDALT,
            13 => MODE13_A::WIREDANDALTFILTER,
            14 => MODE13_A::WIREDANDALTPULLUP,
            15 => MODE13_A::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE13_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE13_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE13_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE13_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE13_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE13_A::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE13_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE13_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE13_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE13_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE13_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE13_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE13_A::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE13_A::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE13_A::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE13_A::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Write proxy for field `MODE13`"]
pub struct MODE13_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE13_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE13_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE13_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE13_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE13_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE13_A::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE13_A::PUSHPULLALT)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE13_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE13_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE13_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE13_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE13_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE13_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE13_A::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE13_A::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE13_A::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE13_A::WIREDANDALTPULLUPFILTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Pin 14 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE14_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output"]
    PUSHPULL = 4,
    #[doc = "5: Push-pull using alternate control"]
    PUSHPULLALT = 5,
    #[doc = "6: Wired-or output"]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output"]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output using alternate control"]
    WIREDANDALT = 12,
    #[doc = "13: Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER = 13,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER = 15,
}
impl From<MODE14_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE14_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE14`"]
pub type MODE14_R = crate::R<u8, MODE14_A>;
impl MODE14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE14_A {
        match self.bits {
            0 => MODE14_A::DISABLED,
            1 => MODE14_A::INPUT,
            2 => MODE14_A::INPUTPULL,
            3 => MODE14_A::INPUTPULLFILTER,
            4 => MODE14_A::PUSHPULL,
            5 => MODE14_A::PUSHPULLALT,
            6 => MODE14_A::WIREDOR,
            7 => MODE14_A::WIREDORPULLDOWN,
            8 => MODE14_A::WIREDAND,
            9 => MODE14_A::WIREDANDFILTER,
            10 => MODE14_A::WIREDANDPULLUP,
            11 => MODE14_A::WIREDANDPULLUPFILTER,
            12 => MODE14_A::WIREDANDALT,
            13 => MODE14_A::WIREDANDALTFILTER,
            14 => MODE14_A::WIREDANDALTPULLUP,
            15 => MODE14_A::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE14_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE14_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE14_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE14_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE14_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE14_A::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE14_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE14_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE14_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE14_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE14_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE14_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE14_A::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE14_A::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE14_A::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE14_A::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Write proxy for field `MODE14`"]
pub struct MODE14_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE14_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE14_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE14_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE14_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE14_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE14_A::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE14_A::PUSHPULLALT)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE14_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE14_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE14_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE14_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE14_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE14_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE14_A::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE14_A::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE14_A::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE14_A::WIREDANDALTPULLUPFILTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Pin 15 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE15_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER = 3,
    #[doc = "4: Push-pull output"]
    PUSHPULL = 4,
    #[doc = "5: Push-pull using alternate control"]
    PUSHPULLALT = 5,
    #[doc = "6: Wired-or output"]
    WIREDOR = 6,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN = 7,
    #[doc = "8: Open-drain output"]
    WIREDAND = 8,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER = 9,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER = 11,
    #[doc = "12: Open-drain output using alternate control"]
    WIREDANDALT = 12,
    #[doc = "13: Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER = 13,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER = 15,
}
impl From<MODE15_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE15_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE15`"]
pub type MODE15_R = crate::R<u8, MODE15_A>;
impl MODE15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE15_A {
        match self.bits {
            0 => MODE15_A::DISABLED,
            1 => MODE15_A::INPUT,
            2 => MODE15_A::INPUTPULL,
            3 => MODE15_A::INPUTPULLFILTER,
            4 => MODE15_A::PUSHPULL,
            5 => MODE15_A::PUSHPULLALT,
            6 => MODE15_A::WIREDOR,
            7 => MODE15_A::WIREDORPULLDOWN,
            8 => MODE15_A::WIREDAND,
            9 => MODE15_A::WIREDANDFILTER,
            10 => MODE15_A::WIREDANDPULLUP,
            11 => MODE15_A::WIREDANDPULLUPFILTER,
            12 => MODE15_A::WIREDANDALT,
            13 => MODE15_A::WIREDANDALTFILTER,
            14 => MODE15_A::WIREDANDALTPULLUP,
            15 => MODE15_A::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE15_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE15_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE15_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE15_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE15_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE15_A::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE15_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE15_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE15_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE15_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE15_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE15_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE15_A::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE15_A::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE15_A::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE15_A::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Write proxy for field `MODE15`"]
pub struct MODE15_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE15_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE15_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE15_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE15_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE15_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE15_A::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE15_A::PUSHPULLALT)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE15_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE15_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE15_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE15_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE15_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE15_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE15_A::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE15_A::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE15_A::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE15_A::WIREDANDALTPULLUPFILTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Pin 8 Mode"]
    #[inline(always)]
    pub fn mode8(&self) -> MODE8_R {
        MODE8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Pin 9 Mode"]
    #[inline(always)]
    pub fn mode9(&self) -> MODE9_R {
        MODE9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Pin 10 Mode"]
    #[inline(always)]
    pub fn mode10(&self) -> MODE10_R {
        MODE10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Pin 11 Mode"]
    #[inline(always)]
    pub fn mode11(&self) -> MODE11_R {
        MODE11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Pin 12 Mode"]
    #[inline(always)]
    pub fn mode12(&self) -> MODE12_R {
        MODE12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Pin 13 Mode"]
    #[inline(always)]
    pub fn mode13(&self) -> MODE13_R {
        MODE13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Pin 14 Mode"]
    #[inline(always)]
    pub fn mode14(&self) -> MODE14_R {
        MODE14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Pin 15 Mode"]
    #[inline(always)]
    pub fn mode15(&self) -> MODE15_R {
        MODE15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pin 8 Mode"]
    #[inline(always)]
    pub fn mode8(&mut self) -> MODE8_W {
        MODE8_W { w: self }
    }
    #[doc = "Bits 4:7 - Pin 9 Mode"]
    #[inline(always)]
    pub fn mode9(&mut self) -> MODE9_W {
        MODE9_W { w: self }
    }
    #[doc = "Bits 8:11 - Pin 10 Mode"]
    #[inline(always)]
    pub fn mode10(&mut self) -> MODE10_W {
        MODE10_W { w: self }
    }
    #[doc = "Bits 12:15 - Pin 11 Mode"]
    #[inline(always)]
    pub fn mode11(&mut self) -> MODE11_W {
        MODE11_W { w: self }
    }
    #[doc = "Bits 16:19 - Pin 12 Mode"]
    #[inline(always)]
    pub fn mode12(&mut self) -> MODE12_W {
        MODE12_W { w: self }
    }
    #[doc = "Bits 20:23 - Pin 13 Mode"]
    #[inline(always)]
    pub fn mode13(&mut self) -> MODE13_W {
        MODE13_W { w: self }
    }
    #[doc = "Bits 24:27 - Pin 14 Mode"]
    #[inline(always)]
    pub fn mode14(&mut self) -> MODE14_W {
        MODE14_W { w: self }
    }
    #[doc = "Bits 28:31 - Pin 15 Mode"]
    #[inline(always)]
    pub fn mode15(&mut self) -> MODE15_W {
        MODE15_W { w: self }
    }
}
