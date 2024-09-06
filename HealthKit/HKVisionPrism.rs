//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKPrismBase(pub NSInteger);
impl HKPrismBase {
    #[doc(alias = "HKPrismBaseNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "HKPrismBaseUp")]
    pub const Up: Self = Self(1);
    #[doc(alias = "HKPrismBaseDown")]
    pub const Down: Self = Self(2);
    #[doc(alias = "HKPrismBaseIn")]
    pub const In: Self = Self(3);
    #[doc(alias = "HKPrismBaseOut")]
    pub const Out: Self = Self(4);
}

unsafe impl Encode for HKPrismBase {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKPrismBase {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKVisionEye(pub NSInteger);
impl HKVisionEye {
    #[doc(alias = "HKVisionEyeLeft")]
    pub const Left: Self = Self(1);
    #[doc(alias = "HKVisionEyeRight")]
    pub const Right: Self = Self(2);
}

unsafe impl Encode for HKVisionEye {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKVisionEye {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKVisionPrism;

    unsafe impl ClassType for HKVisionPrism {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for HKVisionPrism {}

unsafe impl NSCopying for HKVisionPrism {}

unsafe impl CopyingHelper for HKVisionPrism {
    type Result = Self;
}

unsafe impl NSObjectProtocol for HKVisionPrism {}

unsafe impl NSSecureCoding for HKVisionPrism {}

extern_methods!(
    unsafe impl HKVisionPrism {
        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Other amount)]
        pub unsafe fn amount(&self) -> Retained<HKQuantity>;

        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Other angle)]
        pub unsafe fn angle(&self) -> Retained<HKQuantity>;

        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Other verticalAmount)]
        pub unsafe fn verticalAmount(&self) -> Retained<HKQuantity>;

        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Other horizontalAmount)]
        pub unsafe fn horizontalAmount(&self) -> Retained<HKQuantity>;

        #[method(verticalBase)]
        pub unsafe fn verticalBase(&self) -> HKPrismBase;

        #[method(horizontalBase)]
        pub unsafe fn horizontalBase(&self) -> HKPrismBase;

        #[method(eye)]
        pub unsafe fn eye(&self) -> HKVisionEye;

        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Init initWithAmount:angle:eye:)]
        pub unsafe fn initWithAmount_angle_eye(
            this: Allocated<Self>,
            amount: &HKQuantity,
            angle: &HKQuantity,
            eye: HKVisionEye,
        ) -> Retained<Self>;

        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Init initWithVerticalAmount:verticalBase:horizontalAmount:horizontalBase:eye:)]
        pub unsafe fn initWithVerticalAmount_verticalBase_horizontalAmount_horizontalBase_eye(
            this: Allocated<Self>,
            vertical_amount: &HKQuantity,
            vertical_base: HKPrismBase,
            horizontal_amount: &HKQuantity,
            horizontal_base: HKPrismBase,
            eye: HKVisionEye,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
