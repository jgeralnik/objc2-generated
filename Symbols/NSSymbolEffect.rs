//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/symbols/nssymboleffectoptionsrepeatbehavior?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSymbolEffectOptionsRepeatBehavior;
);

unsafe impl NSCoding for NSSymbolEffectOptionsRepeatBehavior {}

unsafe impl NSCopying for NSSymbolEffectOptionsRepeatBehavior {}

unsafe impl CopyingHelper for NSSymbolEffectOptionsRepeatBehavior {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSSymbolEffectOptionsRepeatBehavior {}

unsafe impl NSSecureCoding for NSSymbolEffectOptionsRepeatBehavior {}

extern_methods!(
    unsafe impl NSSymbolEffectOptionsRepeatBehavior {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other behaviorPeriodic)]
        pub unsafe fn behaviorPeriodic() -> Retained<Self>;

        #[method_id(@__retain_semantics Other behaviorPeriodicWithCount:)]
        pub unsafe fn behaviorPeriodicWithCount(count: NSInteger) -> Retained<Self>;

        #[method_id(@__retain_semantics Other behaviorPeriodicWithDelay:)]
        pub unsafe fn behaviorPeriodicWithDelay(delay: c_double) -> Retained<Self>;

        #[method_id(@__retain_semantics Other behaviorPeriodicWithCount:delay:)]
        pub unsafe fn behaviorPeriodicWithCount_delay(
            count: NSInteger,
            delay: c_double,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other behaviorContinuous)]
        pub unsafe fn behaviorContinuous() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/symbols/nssymboleffectoptions?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSymbolEffectOptions;
);

unsafe impl NSCoding for NSSymbolEffectOptions {}

unsafe impl NSCopying for NSSymbolEffectOptions {}

unsafe impl CopyingHelper for NSSymbolEffectOptions {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSSymbolEffectOptions {}

unsafe impl NSSecureCoding for NSSymbolEffectOptions {}

extern_methods!(
    unsafe impl NSSymbolEffectOptions {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other options)]
        pub unsafe fn options() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other optionsWithRepeating)]
        pub unsafe fn optionsWithRepeating_class() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other optionsWithRepeating)]
        pub unsafe fn optionsWithRepeating(&self) -> Retained<Self>;

        #[method_id(@__retain_semantics Other optionsWithNonRepeating)]
        pub unsafe fn optionsWithNonRepeating_class() -> Retained<Self>;

        #[method_id(@__retain_semantics Other optionsWithNonRepeating)]
        pub unsafe fn optionsWithNonRepeating(&self) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other optionsWithRepeatCount:)]
        pub unsafe fn optionsWithRepeatCount_class(count: NSInteger) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other optionsWithRepeatCount:)]
        pub unsafe fn optionsWithRepeatCount(&self, count: NSInteger) -> Retained<Self>;

        #[method_id(@__retain_semantics Other optionsWithSpeed:)]
        pub unsafe fn optionsWithSpeed_class(speed: c_double) -> Retained<Self>;

        #[method_id(@__retain_semantics Other optionsWithSpeed:)]
        pub unsafe fn optionsWithSpeed(&self, speed: c_double) -> Retained<Self>;

        #[method_id(@__retain_semantics Other optionsWithRepeatBehavior:)]
        pub unsafe fn optionsWithRepeatBehavior_class(
            behavior: &NSSymbolEffectOptionsRepeatBehavior,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other optionsWithRepeatBehavior:)]
        pub unsafe fn optionsWithRepeatBehavior(
            &self,
            behavior: &NSSymbolEffectOptionsRepeatBehavior,
        ) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/symbols/nssymboleffect?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSymbolEffect;
);

unsafe impl NSCoding for NSSymbolEffect {}

unsafe impl NSCopying for NSSymbolEffect {}

unsafe impl CopyingHelper for NSSymbolEffect {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSSymbolEffect {}

unsafe impl NSSecureCoding for NSSymbolEffect {}

extern_methods!(
    unsafe impl NSSymbolEffect {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/symbols/nssymbolpulseeffect?language=objc)
    #[unsafe(super(NSSymbolEffect, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSymbolPulseEffect;
);

unsafe impl NSCoding for NSSymbolPulseEffect {}

unsafe impl NSCopying for NSSymbolPulseEffect {}

unsafe impl CopyingHelper for NSSymbolPulseEffect {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSSymbolPulseEffect {}

unsafe impl NSSecureCoding for NSSymbolPulseEffect {}

extern_methods!(
    unsafe impl NSSymbolPulseEffect {
        #[method_id(@__retain_semantics Other effect)]
        pub unsafe fn effect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other effectWithByLayer)]
        pub unsafe fn effectWithByLayer(&self) -> Retained<Self>;

        #[method_id(@__retain_semantics Other effectWithWholeSymbol)]
        pub unsafe fn effectWithWholeSymbol(&self) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSSymbolEffect`
    unsafe impl NSSymbolPulseEffect {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/symbols/nssymbolbounceeffect?language=objc)
    #[unsafe(super(NSSymbolEffect, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSymbolBounceEffect;
);

unsafe impl NSCoding for NSSymbolBounceEffect {}

unsafe impl NSCopying for NSSymbolBounceEffect {}

unsafe impl CopyingHelper for NSSymbolBounceEffect {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSSymbolBounceEffect {}

unsafe impl NSSecureCoding for NSSymbolBounceEffect {}

extern_methods!(
    unsafe impl NSSymbolBounceEffect {
        #[method_id(@__retain_semantics Other effect)]
        pub unsafe fn effect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other bounceUpEffect)]
        pub unsafe fn bounceUpEffect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other bounceDownEffect)]
        pub unsafe fn bounceDownEffect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other effectWithByLayer)]
        pub unsafe fn effectWithByLayer(&self) -> Retained<Self>;

        #[method_id(@__retain_semantics Other effectWithWholeSymbol)]
        pub unsafe fn effectWithWholeSymbol(&self) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSSymbolEffect`
    unsafe impl NSSymbolBounceEffect {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/symbols/nssymbolvariablecoloreffect?language=objc)
    #[unsafe(super(NSSymbolEffect, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSymbolVariableColorEffect;
);

unsafe impl NSCoding for NSSymbolVariableColorEffect {}

unsafe impl NSCopying for NSSymbolVariableColorEffect {}

unsafe impl CopyingHelper for NSSymbolVariableColorEffect {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSSymbolVariableColorEffect {}

unsafe impl NSSecureCoding for NSSymbolVariableColorEffect {}

extern_methods!(
    unsafe impl NSSymbolVariableColorEffect {
        #[method_id(@__retain_semantics Other effect)]
        pub unsafe fn effect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other effectWithIterative)]
        pub unsafe fn effectWithIterative(&self) -> Retained<Self>;

        #[method_id(@__retain_semantics Other effectWithCumulative)]
        pub unsafe fn effectWithCumulative(&self) -> Retained<Self>;

        #[method_id(@__retain_semantics Other effectWithReversing)]
        pub unsafe fn effectWithReversing(&self) -> Retained<Self>;

        #[method_id(@__retain_semantics Other effectWithNonReversing)]
        pub unsafe fn effectWithNonReversing(&self) -> Retained<Self>;

        #[method_id(@__retain_semantics Other effectWithHideInactiveLayers)]
        pub unsafe fn effectWithHideInactiveLayers(&self) -> Retained<Self>;

        #[method_id(@__retain_semantics Other effectWithDimInactiveLayers)]
        pub unsafe fn effectWithDimInactiveLayers(&self) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSSymbolEffect`
    unsafe impl NSSymbolVariableColorEffect {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/symbols/nssymbolscaleeffect?language=objc)
    #[unsafe(super(NSSymbolEffect, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSymbolScaleEffect;
);

unsafe impl NSCoding for NSSymbolScaleEffect {}

unsafe impl NSCopying for NSSymbolScaleEffect {}

unsafe impl CopyingHelper for NSSymbolScaleEffect {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSSymbolScaleEffect {}

unsafe impl NSSecureCoding for NSSymbolScaleEffect {}

extern_methods!(
    unsafe impl NSSymbolScaleEffect {
        #[method_id(@__retain_semantics Other effect)]
        pub unsafe fn effect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other scaleUpEffect)]
        pub unsafe fn scaleUpEffect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other scaleDownEffect)]
        pub unsafe fn scaleDownEffect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other effectWithByLayer)]
        pub unsafe fn effectWithByLayer(&self) -> Retained<Self>;

        #[method_id(@__retain_semantics Other effectWithWholeSymbol)]
        pub unsafe fn effectWithWholeSymbol(&self) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSSymbolEffect`
    unsafe impl NSSymbolScaleEffect {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/symbols/nssymbolappeareffect?language=objc)
    #[unsafe(super(NSSymbolEffect, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSymbolAppearEffect;
);

unsafe impl NSCoding for NSSymbolAppearEffect {}

unsafe impl NSCopying for NSSymbolAppearEffect {}

unsafe impl CopyingHelper for NSSymbolAppearEffect {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSSymbolAppearEffect {}

unsafe impl NSSecureCoding for NSSymbolAppearEffect {}

extern_methods!(
    unsafe impl NSSymbolAppearEffect {
        #[method_id(@__retain_semantics Other effect)]
        pub unsafe fn effect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other appearUpEffect)]
        pub unsafe fn appearUpEffect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other appearDownEffect)]
        pub unsafe fn appearDownEffect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other effectWithByLayer)]
        pub unsafe fn effectWithByLayer(&self) -> Retained<Self>;

        #[method_id(@__retain_semantics Other effectWithWholeSymbol)]
        pub unsafe fn effectWithWholeSymbol(&self) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSSymbolEffect`
    unsafe impl NSSymbolAppearEffect {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/symbols/nssymboldisappeareffect?language=objc)
    #[unsafe(super(NSSymbolEffect, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSymbolDisappearEffect;
);

unsafe impl NSCoding for NSSymbolDisappearEffect {}

unsafe impl NSCopying for NSSymbolDisappearEffect {}

unsafe impl CopyingHelper for NSSymbolDisappearEffect {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSSymbolDisappearEffect {}

unsafe impl NSSecureCoding for NSSymbolDisappearEffect {}

extern_methods!(
    unsafe impl NSSymbolDisappearEffect {
        #[method_id(@__retain_semantics Other effect)]
        pub unsafe fn effect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other disappearUpEffect)]
        pub unsafe fn disappearUpEffect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other disappearDownEffect)]
        pub unsafe fn disappearDownEffect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other effectWithByLayer)]
        pub unsafe fn effectWithByLayer(&self) -> Retained<Self>;

        #[method_id(@__retain_semantics Other effectWithWholeSymbol)]
        pub unsafe fn effectWithWholeSymbol(&self) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSSymbolEffect`
    unsafe impl NSSymbolDisappearEffect {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/symbols/nssymbolwiggleeffect?language=objc)
    #[unsafe(super(NSSymbolEffect, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSymbolWiggleEffect;
);

unsafe impl NSCoding for NSSymbolWiggleEffect {}

unsafe impl NSCopying for NSSymbolWiggleEffect {}

unsafe impl CopyingHelper for NSSymbolWiggleEffect {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSSymbolWiggleEffect {}

unsafe impl NSSecureCoding for NSSymbolWiggleEffect {}

extern_methods!(
    unsafe impl NSSymbolWiggleEffect {
        #[method_id(@__retain_semantics Other effect)]
        pub unsafe fn effect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other wiggleClockwiseEffect)]
        pub unsafe fn wiggleClockwiseEffect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other wiggleCounterClockwiseEffect)]
        pub unsafe fn wiggleCounterClockwiseEffect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other wiggleLeftEffect)]
        pub unsafe fn wiggleLeftEffect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other wiggleRightEffect)]
        pub unsafe fn wiggleRightEffect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other wiggleUpEffect)]
        pub unsafe fn wiggleUpEffect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other wiggleDownEffect)]
        pub unsafe fn wiggleDownEffect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other wiggleForwardEffect)]
        pub unsafe fn wiggleForwardEffect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other wiggleBackwardEffect)]
        pub unsafe fn wiggleBackwardEffect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other wiggleCustomAngleEffect:)]
        pub unsafe fn wiggleCustomAngleEffect(angle: c_double) -> Retained<Self>;

        #[method_id(@__retain_semantics Other effectWithByLayer)]
        pub unsafe fn effectWithByLayer(&self) -> Retained<Self>;

        #[method_id(@__retain_semantics Other effectWithWholeSymbol)]
        pub unsafe fn effectWithWholeSymbol(&self) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSSymbolEffect`
    unsafe impl NSSymbolWiggleEffect {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/symbols/nssymbolrotateeffect?language=objc)
    #[unsafe(super(NSSymbolEffect, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSymbolRotateEffect;
);

unsafe impl NSCoding for NSSymbolRotateEffect {}

unsafe impl NSCopying for NSSymbolRotateEffect {}

unsafe impl CopyingHelper for NSSymbolRotateEffect {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSSymbolRotateEffect {}

unsafe impl NSSecureCoding for NSSymbolRotateEffect {}

extern_methods!(
    unsafe impl NSSymbolRotateEffect {
        #[method_id(@__retain_semantics Other effect)]
        pub unsafe fn effect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other rotateClockwiseEffect)]
        pub unsafe fn rotateClockwiseEffect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other rotateCounterClockwiseEffect)]
        pub unsafe fn rotateCounterClockwiseEffect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other effectWithByLayer)]
        pub unsafe fn effectWithByLayer(&self) -> Retained<Self>;

        #[method_id(@__retain_semantics Other effectWithWholeSymbol)]
        pub unsafe fn effectWithWholeSymbol(&self) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSSymbolEffect`
    unsafe impl NSSymbolRotateEffect {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/symbols/nssymbolbreatheeffect?language=objc)
    #[unsafe(super(NSSymbolEffect, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSymbolBreatheEffect;
);

unsafe impl NSCoding for NSSymbolBreatheEffect {}

unsafe impl NSCopying for NSSymbolBreatheEffect {}

unsafe impl CopyingHelper for NSSymbolBreatheEffect {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSSymbolBreatheEffect {}

unsafe impl NSSecureCoding for NSSymbolBreatheEffect {}

extern_methods!(
    unsafe impl NSSymbolBreatheEffect {
        #[method_id(@__retain_semantics Other effect)]
        pub unsafe fn effect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other breathePulseEffect)]
        pub unsafe fn breathePulseEffect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other breathePlainEffect)]
        pub unsafe fn breathePlainEffect() -> Retained<Self>;

        #[method_id(@__retain_semantics Other effectWithByLayer)]
        pub unsafe fn effectWithByLayer(&self) -> Retained<Self>;

        #[method_id(@__retain_semantics Other effectWithWholeSymbol)]
        pub unsafe fn effectWithWholeSymbol(&self) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSSymbolEffect`
    unsafe impl NSSymbolBreatheEffect {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/symbols/nssymbolcontenttransition?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSymbolContentTransition;
);

unsafe impl NSCoding for NSSymbolContentTransition {}

unsafe impl NSCopying for NSSymbolContentTransition {}

unsafe impl CopyingHelper for NSSymbolContentTransition {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSSymbolContentTransition {}

unsafe impl NSSecureCoding for NSSymbolContentTransition {}

extern_methods!(
    unsafe impl NSSymbolContentTransition {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/symbols/nssymbolmagicreplacecontenttransition?language=objc)
    #[unsafe(super(NSSymbolContentTransition, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSymbolMagicReplaceContentTransition;
);

unsafe impl NSCoding for NSSymbolMagicReplaceContentTransition {}

unsafe impl NSCopying for NSSymbolMagicReplaceContentTransition {}

unsafe impl CopyingHelper for NSSymbolMagicReplaceContentTransition {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSSymbolMagicReplaceContentTransition {}

unsafe impl NSSecureCoding for NSSymbolMagicReplaceContentTransition {}

extern_methods!(
    unsafe impl NSSymbolMagicReplaceContentTransition {}
);

extern_methods!(
    /// Methods declared on superclass `NSSymbolContentTransition`
    unsafe impl NSSymbolMagicReplaceContentTransition {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/symbols/nssymbolreplacecontenttransition?language=objc)
    #[unsafe(super(NSSymbolContentTransition, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSymbolReplaceContentTransition;
);

unsafe impl NSCoding for NSSymbolReplaceContentTransition {}

unsafe impl NSCopying for NSSymbolReplaceContentTransition {}

unsafe impl CopyingHelper for NSSymbolReplaceContentTransition {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSSymbolReplaceContentTransition {}

unsafe impl NSSecureCoding for NSSymbolReplaceContentTransition {}

extern_methods!(
    unsafe impl NSSymbolReplaceContentTransition {
        #[method_id(@__retain_semantics Other transition)]
        pub unsafe fn transition() -> Retained<Self>;

        #[method_id(@__retain_semantics Other replaceDownUpTransition)]
        pub unsafe fn replaceDownUpTransition() -> Retained<Self>;

        #[method_id(@__retain_semantics Other replaceUpUpTransition)]
        pub unsafe fn replaceUpUpTransition() -> Retained<Self>;

        #[method_id(@__retain_semantics Other replaceOffUpTransition)]
        pub unsafe fn replaceOffUpTransition() -> Retained<Self>;

        #[method_id(@__retain_semantics Other transitionWithByLayer)]
        pub unsafe fn transitionWithByLayer(&self) -> Retained<Self>;

        #[method_id(@__retain_semantics Other transitionWithWholeSymbol)]
        pub unsafe fn transitionWithWholeSymbol(&self) -> Retained<Self>;

        #[method_id(@__retain_semantics Other magicTransitionWithFallback:)]
        pub unsafe fn magicTransitionWithFallback(
            fallback: &NSSymbolReplaceContentTransition,
        ) -> Retained<NSSymbolMagicReplaceContentTransition>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSSymbolContentTransition`
    unsafe impl NSSymbolReplaceContentTransition {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/symbols/nssymbolautomaticcontenttransition?language=objc)
    #[unsafe(super(NSSymbolContentTransition, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSymbolAutomaticContentTransition;
);

unsafe impl NSCoding for NSSymbolAutomaticContentTransition {}

unsafe impl NSCopying for NSSymbolAutomaticContentTransition {}

unsafe impl CopyingHelper for NSSymbolAutomaticContentTransition {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSSymbolAutomaticContentTransition {}

unsafe impl NSSecureCoding for NSSymbolAutomaticContentTransition {}

extern_methods!(
    unsafe impl NSSymbolAutomaticContentTransition {
        #[method_id(@__retain_semantics Other transition)]
        pub unsafe fn transition() -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSSymbolContentTransition`
    unsafe impl NSSymbolAutomaticContentTransition {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
