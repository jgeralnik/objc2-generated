//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CMMotionActivityConfidence(pub NSInteger);
impl CMMotionActivityConfidence {
    #[doc(alias = "CMMotionActivityConfidenceLow")]
    pub const Low: Self = Self(0);
    #[doc(alias = "CMMotionActivityConfidenceMedium")]
    pub const Medium: Self = Self(1);
    #[doc(alias = "CMMotionActivityConfidenceHigh")]
    pub const High: Self = Self(2);
}

unsafe impl Encode for CMMotionActivityConfidence {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CMMotionActivityConfidence {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CMLogItem")]
    pub struct CMMotionActivity;

    #[cfg(feature = "CMLogItem")]
    unsafe impl ClassType for CMMotionActivity {
        #[inherits(NSObject)]
        type Super = CMLogItem;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CMLogItem")]
unsafe impl NSCoding for CMMotionActivity {}

#[cfg(feature = "CMLogItem")]
unsafe impl NSCopying for CMMotionActivity {}

#[cfg(feature = "CMLogItem")]
unsafe impl NSObjectProtocol for CMMotionActivity {}

#[cfg(feature = "CMLogItem")]
unsafe impl NSSecureCoding for CMMotionActivity {}

extern_methods!(
    #[cfg(feature = "CMLogItem")]
    unsafe impl CMMotionActivity {
        #[method(confidence)]
        pub unsafe fn confidence(&self) -> CMMotionActivityConfidence;

        #[method_id(@__retain_semantics Other startDate)]
        pub unsafe fn startDate(&self) -> Id<NSDate>;

        #[method(unknown)]
        pub unsafe fn unknown(&self) -> bool;

        #[method(stationary)]
        pub unsafe fn stationary(&self) -> bool;

        #[method(walking)]
        pub unsafe fn walking(&self) -> bool;

        #[method(running)]
        pub unsafe fn running(&self) -> bool;

        #[method(automotive)]
        pub unsafe fn automotive(&self) -> bool;

        #[method(cycling)]
        pub unsafe fn cycling(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CMLogItem")]
    unsafe impl CMMotionActivity {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
