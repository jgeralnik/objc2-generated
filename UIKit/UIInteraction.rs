//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiinteraction?language=objc)
    pub unsafe trait UIInteraction: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other view)]
        unsafe fn view(&self) -> Option<Retained<UIView>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method(willMoveToView:)]
        unsafe fn willMoveToView(&self, view: Option<&UIView>);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method(didMoveToView:)]
        unsafe fn didMoveToView(&self, view: Option<&UIView>);
    }

    unsafe impl ProtocolType for dyn UIInteraction {}
);

extern_methods!(
    /// Interactions
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIView {
        #[method(addInteraction:)]
        pub unsafe fn addInteraction(&self, interaction: &ProtocolObject<dyn UIInteraction>);

        #[method(removeInteraction:)]
        pub unsafe fn removeInteraction(&self, interaction: &ProtocolObject<dyn UIInteraction>);

        #[method_id(@__retain_semantics Other interactions)]
        pub unsafe fn interactions(&self) -> Retained<NSArray<ProtocolObject<dyn UIInteraction>>>;

        #[method(setInteractions:)]
        pub unsafe fn setInteractions(
            &self,
            interactions: &NSArray<ProtocolObject<dyn UIInteraction>>,
        );
    }
);
