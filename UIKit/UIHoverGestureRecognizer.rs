//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uihovergesturerecognizer?language=objc)
    #[unsafe(super(UIGestureRecognizer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIGestureRecognizer")]
    pub struct UIHoverGestureRecognizer;
);

#[cfg(feature = "UIGestureRecognizer")]
unsafe impl NSObjectProtocol for UIHoverGestureRecognizer {}

extern_methods!(
    #[cfg(feature = "UIGestureRecognizer")]
    unsafe impl UIHoverGestureRecognizer {
        #[method(zOffset)]
        pub unsafe fn zOffset(&self) -> CGFloat;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method(azimuthAngleInView:)]
        pub unsafe fn azimuthAngleInView(&self, view: Option<&UIView>) -> CGFloat;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method(azimuthUnitVectorInView:)]
        pub unsafe fn azimuthUnitVectorInView(&self, view: Option<&UIView>) -> CGVector;

        #[method(altitudeAngle)]
        pub unsafe fn altitudeAngle(&self) -> CGFloat;

        #[method(rollAngle)]
        pub unsafe fn rollAngle(&self) -> CGFloat;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIGestureRecognizer`
    #[cfg(feature = "UIGestureRecognizer")]
    unsafe impl UIHoverGestureRecognizer {
        #[method_id(@__retain_semantics Init initWithTarget:action:)]
        pub unsafe fn initWithTarget_action(
            this: Allocated<Self>,
            target: Option<&AnyObject>,
            action: Option<Sel>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIGestureRecognizer")]
    unsafe impl UIHoverGestureRecognizer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
