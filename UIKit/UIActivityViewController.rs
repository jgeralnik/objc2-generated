//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[cfg(all(feature = "UIActivity", feature = "block2"))]
pub type UIActivityViewControllerCompletionHandler =
    *mut block2::Block<dyn Fn(*mut UIActivityType, Bool)>;

#[cfg(all(feature = "UIActivity", feature = "block2"))]
pub type UIActivityViewControllerCompletionWithItemsHandler =
    *mut block2::Block<dyn Fn(*mut UIActivityType, Bool, *mut NSArray, *mut NSError)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    pub struct UIActivityViewController;

    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl ClassType for UIActivityViewController {
        #[inherits(UIResponder, NSObject)]
        type Super = UIViewController;
    }
);

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSCoding for UIActivityViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSObjectProtocol for UIActivityViewController {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIAppearanceContainer for UIActivityViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIContentContainer for UIActivityViewController {}

#[cfg(all(
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIFocusEnvironment for UIActivityViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIResponderStandardEditActions for UIActivityViewController {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIViewController"
))]
unsafe impl UITraitEnvironment for UIActivityViewController {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIActivityViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSString>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "UIActivity")]
        #[method_id(@__retain_semantics Init initWithActivityItems:applicationActivities:)]
        pub unsafe fn initWithActivityItems_applicationActivities(
            this: Allocated<Self>,
            activity_items: &NSArray,
            application_activities: Option<&NSArray<UIActivity>>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "UIActivity", feature = "block2"))]
        #[deprecated]
        #[method(completionHandler)]
        pub unsafe fn completionHandler(&self) -> UIActivityViewControllerCompletionHandler;

        #[cfg(all(feature = "UIActivity", feature = "block2"))]
        #[deprecated]
        #[method(setCompletionHandler:)]
        pub unsafe fn setCompletionHandler(
            &self,
            completion_handler: UIActivityViewControllerCompletionHandler,
        );

        #[cfg(all(feature = "UIActivity", feature = "block2"))]
        #[method(completionWithItemsHandler)]
        pub unsafe fn completionWithItemsHandler(
            &self,
        ) -> UIActivityViewControllerCompletionWithItemsHandler;

        #[cfg(all(feature = "UIActivity", feature = "block2"))]
        #[method(setCompletionWithItemsHandler:)]
        pub unsafe fn setCompletionWithItemsHandler(
            &self,
            completion_with_items_handler: UIActivityViewControllerCompletionWithItemsHandler,
        );

        #[cfg(feature = "UIActivity")]
        #[method_id(@__retain_semantics Other excludedActivityTypes)]
        pub unsafe fn excludedActivityTypes(&self) -> Option<Retained<NSArray<UIActivityType>>>;

        #[cfg(feature = "UIActivity")]
        #[method(setExcludedActivityTypes:)]
        pub unsafe fn setExcludedActivityTypes(
            &self,
            excluded_activity_types: Option<&NSArray<UIActivityType>>,
        );

        #[method(allowsProminentActivity)]
        pub unsafe fn allowsProminentActivity(&self) -> bool;

        #[method(setAllowsProminentActivity:)]
        pub unsafe fn setAllowsProminentActivity(&self, allows_prominent_activity: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIActivityViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// UIActivityItemsConfiguration
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIActivityViewController {
        #[cfg(feature = "UIActivityItemsConfigurationReading")]
        #[method_id(@__retain_semantics Init initWithActivityItemsConfiguration:)]
        pub unsafe fn initWithActivityItemsConfiguration(
            this: Allocated<Self>,
            activity_items_configuration: &ProtocolObject<dyn UIActivityItemsConfigurationReading>,
        ) -> Retained<Self>;
    }
);
