//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIWindowSceneResizingRestrictions(pub NSInteger);
impl UIWindowSceneResizingRestrictions {
    #[doc(alias = "UIWindowSceneResizingRestrictionsUnspecified")]
    pub const Unspecified: Self = Self(0);
    #[doc(alias = "UIWindowSceneResizingRestrictionsNone")]
    pub const None: Self = Self(1);
    #[doc(alias = "UIWindowSceneResizingRestrictionsUniform")]
    pub const Uniform: Self = Self(2);
    #[doc(alias = "UIWindowSceneResizingRestrictionsFreeform")]
    pub const Freeform: Self = Self(3);
}

unsafe impl Encode for UIWindowSceneResizingRestrictions {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIWindowSceneResizingRestrictions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIWindowSceneGeometry;

    unsafe impl ClassType for UIWindowSceneGeometry {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for UIWindowSceneGeometry {}

unsafe impl CopyingHelper for UIWindowSceneGeometry {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIWindowSceneGeometry {}

extern_methods!(
    unsafe impl UIWindowSceneGeometry {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method(systemFrame)]
        pub unsafe fn systemFrame(&self) -> CGRect;

        #[cfg(feature = "UIOrientation")]
        #[method(interfaceOrientation)]
        pub unsafe fn interfaceOrientation(&self) -> UIInterfaceOrientation;

        #[method(minimumSize)]
        pub unsafe fn minimumSize(&self) -> CGSize;

        #[method(maximumSize)]
        pub unsafe fn maximumSize(&self) -> CGSize;

        #[method(resizingRestrictions)]
        pub unsafe fn resizingRestrictions(&self) -> UIWindowSceneResizingRestrictions;
    }
);
