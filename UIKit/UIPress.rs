//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipressphase?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIPressPhase(pub NSInteger);
impl UIPressPhase {
    #[doc(alias = "UIPressPhaseBegan")]
    pub const Began: Self = Self(0);
    #[doc(alias = "UIPressPhaseChanged")]
    pub const Changed: Self = Self(1);
    #[doc(alias = "UIPressPhaseStationary")]
    pub const Stationary: Self = Self(2);
    #[doc(alias = "UIPressPhaseEnded")]
    pub const Ended: Self = Self(3);
    #[doc(alias = "UIPressPhaseCancelled")]
    pub const Cancelled: Self = Self(4);
}

unsafe impl Encode for UIPressPhase {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIPressPhase {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipresstype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIPressType(pub NSInteger);
impl UIPressType {
    #[doc(alias = "UIPressTypeUpArrow")]
    pub const UpArrow: Self = Self(0);
    #[doc(alias = "UIPressTypeDownArrow")]
    pub const DownArrow: Self = Self(1);
    #[doc(alias = "UIPressTypeLeftArrow")]
    pub const LeftArrow: Self = Self(2);
    #[doc(alias = "UIPressTypeRightArrow")]
    pub const RightArrow: Self = Self(3);
    #[doc(alias = "UIPressTypeSelect")]
    pub const Select: Self = Self(4);
    #[doc(alias = "UIPressTypeMenu")]
    pub const Menu: Self = Self(5);
    #[doc(alias = "UIPressTypePlayPause")]
    pub const PlayPause: Self = Self(6);
    #[doc(alias = "UIPressTypePageUp")]
    pub const PageUp: Self = Self(30);
    #[doc(alias = "UIPressTypePageDown")]
    pub const PageDown: Self = Self(31);
    #[doc(alias = "UIPressTypeTVRemoteOneTwoThree")]
    pub const TVRemoteOneTwoThree: Self = Self(32);
    #[doc(alias = "UIPressTypeTVRemoteFourColors")]
    pub const TVRemoteFourColors: Self = Self(33);
}

unsafe impl Encode for UIPressType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIPressType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipress?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPress;
);

unsafe impl NSObjectProtocol for UIPress {}

extern_methods!(
    unsafe impl UIPress {
        #[method(timestamp)]
        pub unsafe fn timestamp(&self) -> NSTimeInterval;

        #[method(phase)]
        pub unsafe fn phase(&self) -> UIPressPhase;

        #[method(type)]
        pub unsafe fn r#type(&self) -> UIPressType;

        #[cfg(all(feature = "UIResponder", feature = "UIView", feature = "UIWindow"))]
        #[method_id(@__retain_semantics Other window)]
        pub unsafe fn window(&self, mtm: MainThreadMarker) -> Option<Retained<UIWindow>>;

        #[cfg(feature = "UIResponder")]
        #[method_id(@__retain_semantics Other responder)]
        pub unsafe fn responder(&self, mtm: MainThreadMarker) -> Option<Retained<UIResponder>>;

        #[cfg(feature = "UIGestureRecognizer")]
        #[method_id(@__retain_semantics Other gestureRecognizers)]
        pub unsafe fn gestureRecognizers(
            &self,
            mtm: MainThreadMarker,
        ) -> Option<Retained<NSArray<UIGestureRecognizer>>>;

        #[method(force)]
        pub unsafe fn force(&self) -> CGFloat;

        #[cfg(feature = "UIKey")]
        #[method_id(@__retain_semantics Other key)]
        pub unsafe fn key(&self, mtm: MainThreadMarker) -> Option<Retained<UIKey>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIPress {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
