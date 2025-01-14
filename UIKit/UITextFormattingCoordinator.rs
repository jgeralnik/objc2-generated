//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingcoordinatordelegate?language=objc)
    pub unsafe trait UITextFormattingCoordinatorDelegate:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(all(feature = "UIResponder", feature = "block2"))]
        #[method(updateTextAttributesWithConversionHandler:)]
        unsafe fn updateTextAttributesWithConversionHandler(
            &self,
            conversion_handler: UITextAttributesConversionHandler,
        );
    }

    unsafe impl ProtocolType for dyn UITextFormattingCoordinatorDelegate {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextformattingcoordinator?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITextFormattingCoordinator;
);

unsafe impl NSObjectProtocol for UITextFormattingCoordinator {}

#[cfg(feature = "UIFontPickerViewController")]
unsafe impl UIFontPickerViewControllerDelegate for UITextFormattingCoordinator {}

extern_methods!(
    unsafe impl UITextFormattingCoordinator {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UITextFormattingCoordinatorDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UITextFormattingCoordinatorDelegate>>,
        );

        #[method(isFontPanelVisible)]
        pub unsafe fn isFontPanelVisible(mtm: MainThreadMarker) -> bool;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIScene",
            feature = "UIWindowScene"
        ))]
        #[method_id(@__retain_semantics Other textFormattingCoordinatorForWindowScene:)]
        pub unsafe fn textFormattingCoordinatorForWindowScene(
            window_scene: &UIWindowScene,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIScene",
            feature = "UIWindowScene"
        ))]
        #[method_id(@__retain_semantics Init initWithWindowScene:)]
        pub unsafe fn initWithWindowScene(
            this: Allocated<Self>,
            window_scene: &UIWindowScene,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(setSelectedAttributes:isMultiple:)]
        pub unsafe fn setSelectedAttributes_isMultiple(
            &self,
            attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
            flag: bool,
        );

        #[method(toggleFontPanel:)]
        pub unsafe fn toggleFontPanel(sender: &AnyObject, mtm: MainThreadMarker);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITextFormattingCoordinator {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
