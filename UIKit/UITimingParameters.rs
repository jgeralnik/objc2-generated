//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicubictimingparameters?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UICubicTimingParameters;
);

unsafe impl NSCoding for UICubicTimingParameters {}

unsafe impl NSCopying for UICubicTimingParameters {}

unsafe impl CopyingHelper for UICubicTimingParameters {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UICubicTimingParameters {}

#[cfg(feature = "UITimingCurveProvider")]
unsafe impl UITimingCurveProvider for UICubicTimingParameters {}

extern_methods!(
    unsafe impl UICubicTimingParameters {
        #[cfg(feature = "UIView")]
        #[method(animationCurve)]
        pub unsafe fn animationCurve(&self) -> UIViewAnimationCurve;

        #[method(controlPoint1)]
        pub unsafe fn controlPoint1(&self) -> CGPoint;

        #[method(controlPoint2)]
        pub unsafe fn controlPoint2(&self) -> CGPoint;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "UIView")]
        #[method_id(@__retain_semantics Init initWithAnimationCurve:)]
        pub unsafe fn initWithAnimationCurve(
            this: Allocated<Self>,
            curve: UIViewAnimationCurve,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithControlPoint1:controlPoint2:)]
        pub unsafe fn initWithControlPoint1_controlPoint2(
            this: Allocated<Self>,
            point1: CGPoint,
            point2: CGPoint,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UICubicTimingParameters {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uispringtimingparameters?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UISpringTimingParameters;
);

unsafe impl NSCoding for UISpringTimingParameters {}

unsafe impl NSCopying for UISpringTimingParameters {}

unsafe impl CopyingHelper for UISpringTimingParameters {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UISpringTimingParameters {}

#[cfg(feature = "UITimingCurveProvider")]
unsafe impl UITimingCurveProvider for UISpringTimingParameters {}

extern_methods!(
    unsafe impl UISpringTimingParameters {
        #[method(initialVelocity)]
        pub unsafe fn initialVelocity(&self) -> CGVector;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithDampingRatio:initialVelocity:)]
        pub unsafe fn initWithDampingRatio_initialVelocity(
            this: Allocated<Self>,
            ratio: CGFloat,
            velocity: CGVector,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithMass:stiffness:damping:initialVelocity:)]
        pub unsafe fn initWithMass_stiffness_damping_initialVelocity(
            this: Allocated<Self>,
            mass: CGFloat,
            stiffness: CGFloat,
            damping: CGFloat,
            velocity: CGVector,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDampingRatio:)]
        pub unsafe fn initWithDampingRatio(this: Allocated<Self>, ratio: CGFloat)
            -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDuration:bounce:initialVelocity:)]
        pub unsafe fn initWithDuration_bounce_initialVelocity(
            this: Allocated<Self>,
            duration: NSTimeInterval,
            bounce: CGFloat,
            velocity: CGVector,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDuration:bounce:)]
        pub unsafe fn initWithDuration_bounce(
            this: Allocated<Self>,
            duration: NSTimeInterval,
            bounce: CGFloat,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UISpringTimingParameters {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
