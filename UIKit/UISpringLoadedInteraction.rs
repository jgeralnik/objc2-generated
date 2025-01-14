//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uispringloadedinteractioneffectstate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UISpringLoadedInteractionEffectState(pub NSInteger);
impl UISpringLoadedInteractionEffectState {
    #[doc(alias = "UISpringLoadedInteractionEffectStateInactive")]
    pub const Inactive: Self = Self(0);
    #[doc(alias = "UISpringLoadedInteractionEffectStatePossible")]
    pub const Possible: Self = Self(1);
    #[doc(alias = "UISpringLoadedInteractionEffectStateActivating")]
    pub const Activating: Self = Self(2);
    #[doc(alias = "UISpringLoadedInteractionEffectStateActivated")]
    pub const Activated: Self = Self(3);
}

unsafe impl Encode for UISpringLoadedInteractionEffectState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UISpringLoadedInteractionEffectState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uispringloadedinteraction?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UISpringLoadedInteraction;
);

unsafe impl NSObjectProtocol for UISpringLoadedInteraction {}

#[cfg(feature = "UIInteraction")]
unsafe impl UIInteraction for UISpringLoadedInteraction {}

extern_methods!(
    unsafe impl UISpringLoadedInteraction {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithInteractionBehavior:interactionEffect:activationHandler:)]
        pub unsafe fn initWithInteractionBehavior_interactionEffect_activationHandler(
            this: Allocated<Self>,
            interaction_behavior: Option<&ProtocolObject<dyn UISpringLoadedInteractionBehavior>>,
            interaction_effect: Option<&ProtocolObject<dyn UISpringLoadedInteractionEffect>>,
            handler: &block2::Block<
                dyn Fn(
                    NonNull<UISpringLoadedInteraction>,
                    NonNull<ProtocolObject<dyn UISpringLoadedInteractionContext>>,
                ),
            >,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithActivationHandler:)]
        pub unsafe fn initWithActivationHandler(
            this: Allocated<Self>,
            handler: &block2::Block<
                dyn Fn(
                    NonNull<UISpringLoadedInteraction>,
                    NonNull<ProtocolObject<dyn UISpringLoadedInteractionContext>>,
                ),
            >,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other interactionBehavior)]
        pub unsafe fn interactionBehavior(
            &self,
        ) -> Retained<ProtocolObject<dyn UISpringLoadedInteractionBehavior>>;

        #[method_id(@__retain_semantics Other interactionEffect)]
        pub unsafe fn interactionEffect(
            &self,
        ) -> Retained<ProtocolObject<dyn UISpringLoadedInteractionEffect>>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uispringloadedinteractionbehavior?language=objc)
    pub unsafe trait UISpringLoadedInteractionBehavior:
        NSObjectProtocol + MainThreadOnly
    {
        #[method(shouldAllowInteraction:withContext:)]
        unsafe fn shouldAllowInteraction_withContext(
            &self,
            interaction: &UISpringLoadedInteraction,
            context: &ProtocolObject<dyn UISpringLoadedInteractionContext>,
        ) -> bool;

        #[optional]
        #[method(interactionDidFinish:)]
        unsafe fn interactionDidFinish(&self, interaction: &UISpringLoadedInteraction);
    }

    unsafe impl ProtocolType for dyn UISpringLoadedInteractionBehavior {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uispringloadedinteractioneffect?language=objc)
    pub unsafe trait UISpringLoadedInteractionEffect:
        NSObjectProtocol + MainThreadOnly
    {
        #[method(interaction:didChangeWithContext:)]
        unsafe fn interaction_didChangeWithContext(
            &self,
            interaction: &UISpringLoadedInteraction,
            context: &ProtocolObject<dyn UISpringLoadedInteractionContext>,
        );
    }

    unsafe impl ProtocolType for dyn UISpringLoadedInteractionEffect {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uispringloadedinteractioncontext?language=objc)
    pub unsafe trait UISpringLoadedInteractionContext:
        NSObjectProtocol + MainThreadOnly
    {
        #[method(state)]
        unsafe fn state(&self) -> UISpringLoadedInteractionEffectState;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other targetView)]
        unsafe fn targetView(&self) -> Option<Retained<UIView>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method(setTargetView:)]
        unsafe fn setTargetView(&self, target_view: Option<&UIView>);

        #[method_id(@__retain_semantics Other targetItem)]
        unsafe fn targetItem(&self) -> Option<Retained<AnyObject>>;

        #[method(setTargetItem:)]
        unsafe fn setTargetItem(&self, target_item: Option<&AnyObject>);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method(locationInView:)]
        unsafe fn locationInView(&self, view: Option<&UIView>) -> CGPoint;
    }

    unsafe impl ProtocolType for dyn UISpringLoadedInteractionContext {}
);
