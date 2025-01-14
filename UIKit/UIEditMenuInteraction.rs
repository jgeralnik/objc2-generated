//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uieditmenuarrowdirection?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIEditMenuArrowDirection(pub NSInteger);
impl UIEditMenuArrowDirection {
    #[doc(alias = "UIEditMenuArrowDirectionAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "UIEditMenuArrowDirectionUp")]
    pub const Up: Self = Self(1);
    #[doc(alias = "UIEditMenuArrowDirectionDown")]
    pub const Down: Self = Self(2);
    #[doc(alias = "UIEditMenuArrowDirectionLeft")]
    pub const Left: Self = Self(3);
    #[doc(alias = "UIEditMenuArrowDirectionRight")]
    pub const Right: Self = Self(4);
}

unsafe impl Encode for UIEditMenuArrowDirection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIEditMenuArrowDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uieditmenuconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIEditMenuConfiguration;
);

unsafe impl NSObjectProtocol for UIEditMenuConfiguration {}

extern_methods!(
    unsafe impl UIEditMenuConfiguration {
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<ProtocolObject<dyn NSCopying>>;

        #[method(sourcePoint)]
        pub unsafe fn sourcePoint(&self) -> CGPoint;

        #[method(preferredArrowDirection)]
        pub unsafe fn preferredArrowDirection(&self) -> UIEditMenuArrowDirection;

        #[method(setPreferredArrowDirection:)]
        pub unsafe fn setPreferredArrowDirection(
            &self,
            preferred_arrow_direction: UIEditMenuArrowDirection,
        );

        #[method_id(@__retain_semantics Other configurationWithIdentifier:sourcePoint:)]
        pub unsafe fn configurationWithIdentifier_sourcePoint(
            identifier: Option<&ProtocolObject<dyn NSCopying>>,
            source_point: CGPoint,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uieditmenuinteraction?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIEditMenuInteraction;
);

unsafe impl NSObjectProtocol for UIEditMenuInteraction {}

#[cfg(feature = "UIInteraction")]
unsafe impl UIInteraction for UIEditMenuInteraction {}

extern_methods!(
    unsafe impl UIEditMenuInteraction {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIEditMenuInteractionDelegate>>>;

        #[method_id(@__retain_semantics Init initWithDelegate:)]
        pub unsafe fn initWithDelegate(
            this: Allocated<Self>,
            delegate: Option<&ProtocolObject<dyn UIEditMenuInteractionDelegate>>,
        ) -> Retained<Self>;

        #[method(presentEditMenuWithConfiguration:)]
        pub unsafe fn presentEditMenuWithConfiguration(
            &self,
            configuration: &UIEditMenuConfiguration,
        );

        #[method(dismissMenu)]
        pub unsafe fn dismissMenu(&self);

        #[method(reloadVisibleMenu)]
        pub unsafe fn reloadVisibleMenu(&self);

        #[method(updateVisibleMenuPositionAnimated:)]
        pub unsafe fn updateVisibleMenuPositionAnimated(&self, animated: bool);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method(locationInView:)]
        pub unsafe fn locationInView(&self, view: Option<&UIView>) -> CGPoint;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uieditmenuinteractionanimating?language=objc)
    pub unsafe trait UIEditMenuInteractionAnimating:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(feature = "block2")]
        #[method(addAnimations:)]
        unsafe fn addAnimations(&self, animations: &block2::Block<dyn Fn()>);

        #[cfg(feature = "block2")]
        #[method(addCompletion:)]
        unsafe fn addCompletion(&self, completion: &block2::Block<dyn Fn()>);
    }

    unsafe impl ProtocolType for dyn UIEditMenuInteractionAnimating {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uieditmenuinteractiondelegate?language=objc)
    pub unsafe trait UIEditMenuInteractionDelegate: NSObjectProtocol {
        #[cfg(all(feature = "UIMenu", feature = "UIMenuElement"))]
        #[optional]
        #[method_id(@__retain_semantics Other editMenuInteraction:menuForConfiguration:suggestedActions:)]
        unsafe fn editMenuInteraction_menuForConfiguration_suggestedActions(
            &self,
            interaction: &UIEditMenuInteraction,
            configuration: &UIEditMenuConfiguration,
            suggested_actions: &NSArray<UIMenuElement>,
        ) -> Option<Retained<UIMenu>>;

        #[optional]
        #[method(editMenuInteraction:targetRectForConfiguration:)]
        unsafe fn editMenuInteraction_targetRectForConfiguration(
            &self,
            interaction: &UIEditMenuInteraction,
            configuration: &UIEditMenuConfiguration,
        ) -> CGRect;

        #[optional]
        #[method(editMenuInteraction:willPresentMenuForConfiguration:animator:)]
        unsafe fn editMenuInteraction_willPresentMenuForConfiguration_animator(
            &self,
            interaction: &UIEditMenuInteraction,
            configuration: &UIEditMenuConfiguration,
            animator: &ProtocolObject<dyn UIEditMenuInteractionAnimating>,
        );

        #[optional]
        #[method(editMenuInteraction:willDismissMenuForConfiguration:animator:)]
        unsafe fn editMenuInteraction_willDismissMenuForConfiguration_animator(
            &self,
            interaction: &UIEditMenuInteraction,
            configuration: &UIEditMenuConfiguration,
            animator: &ProtocolObject<dyn UIEditMenuInteractionAnimating>,
        );
    }

    unsafe impl ProtocolType for dyn UIEditMenuInteractionDelegate {}
);
