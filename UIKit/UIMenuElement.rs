//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uimenuelementstate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIMenuElementState(pub NSInteger);
impl UIMenuElementState {
    #[doc(alias = "UIMenuElementStateOff")]
    pub const Off: Self = Self(0);
    #[doc(alias = "UIMenuElementStateOn")]
    pub const On: Self = Self(1);
    #[doc(alias = "UIMenuElementStateMixed")]
    pub const Mixed: Self = Self(2);
}

unsafe impl Encode for UIMenuElementState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIMenuElementState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uimenuelementattributes?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIMenuElementAttributes(pub NSUInteger);
bitflags::bitflags! {
    impl UIMenuElementAttributes: NSUInteger {
        #[doc(alias = "UIMenuElementAttributesDisabled")]
        const Disabled = 1<<0;
        #[doc(alias = "UIMenuElementAttributesDestructive")]
        const Destructive = 1<<1;
        #[doc(alias = "UIMenuElementAttributesHidden")]
        const Hidden = 1<<2;
        #[doc(alias = "UIMenuElementAttributesKeepsMenuPresented")]
        const KeepsMenuPresented = 1<<3;
    }
}

unsafe impl Encode for UIMenuElementAttributes {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIMenuElementAttributes {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uimenuelement?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIMenuElement;
);

unsafe impl NSCoding for UIMenuElement {}

unsafe impl NSCopying for UIMenuElement {}

unsafe impl CopyingHelper for UIMenuElement {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIMenuElement {}

unsafe impl NSSecureCoding for UIMenuElement {}

extern_methods!(
    unsafe impl UIMenuElement {
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other subtitle)]
        pub unsafe fn subtitle(&self) -> Option<Retained<NSString>>;

        #[method(setSubtitle:)]
        pub unsafe fn setSubtitle(&self, subtitle: Option<&NSString>);

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Retained<UIImage>>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
