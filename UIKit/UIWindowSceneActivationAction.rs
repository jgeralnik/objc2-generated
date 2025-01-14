//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiwindowsceneactivationactionconfigurationprovider?language=objc)
#[cfg(all(
    feature = "UIAction",
    feature = "UIMenuElement",
    feature = "UIWindowSceneActivationConfiguration",
    feature = "block2"
))]
pub type UIWindowSceneActivationActionConfigurationProvider = *mut block2::Block<
    dyn Fn(NonNull<UIWindowSceneActivationAction>) -> *mut UIWindowSceneActivationConfiguration,
>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiwindowsceneactivationaction?language=objc)
    #[unsafe(super(UIAction, UIMenuElement, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
    pub struct UIWindowSceneActivationAction;
);

#[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
unsafe impl NSCoding for UIWindowSceneActivationAction {}

#[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
unsafe impl NSCopying for UIWindowSceneActivationAction {}

#[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
unsafe impl CopyingHelper for UIWindowSceneActivationAction {
    type Result = Self;
}

#[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
unsafe impl NSObjectProtocol for UIWindowSceneActivationAction {}

#[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
unsafe impl NSSecureCoding for UIWindowSceneActivationAction {}

#[cfg(all(
    feature = "UIAction",
    feature = "UIMenuElement",
    feature = "UIMenuLeaf"
))]
unsafe impl UIMenuLeaf for UIWindowSceneActivationAction {}

extern_methods!(
    #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
    unsafe impl UIWindowSceneActivationAction {
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[cfg(all(feature = "UIWindowSceneActivationConfiguration", feature = "block2"))]
        #[method_id(@__retain_semantics Other actionWithIdentifier:alternateAction:configurationProvider:)]
        pub unsafe fn actionWithIdentifier_alternateAction_configurationProvider(
            identifier: Option<&UIActionIdentifier>,
            alternate_action: Option<&UIAction>,
            configuration_provider: UIWindowSceneActivationActionConfigurationProvider,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other actionWithHandler:)]
        pub unsafe fn actionWithHandler(
            handler: UIActionHandler,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(all(feature = "UIImage", feature = "block2"))]
        #[method_id(@__retain_semantics Other actionWithTitle:image:identifier:handler:)]
        pub unsafe fn actionWithTitle_image_identifier_handler(
            title: &NSString,
            image: Option<&UIImage>,
            identifier: Option<&UIActionIdentifier>,
            handler: UIActionHandler,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIAction`
    #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
    unsafe impl UIWindowSceneActivationAction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIMenuElement`
    #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
    unsafe impl UIWindowSceneActivationAction {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);
