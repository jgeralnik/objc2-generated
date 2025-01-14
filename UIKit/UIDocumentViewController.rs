//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidocumentviewcontroller?language=objc)
    #[unsafe(super(UIViewController, UIResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    pub struct UIDocumentViewController;
);

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSCoding for UIDocumentViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSObjectProtocol for UIDocumentViewController {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIAppearanceContainer for UIDocumentViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIContentContainer for UIDocumentViewController {}

#[cfg(all(
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIFocusEnvironment for UIDocumentViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIResponderStandardEditActions for UIDocumentViewController {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIViewController"
))]
unsafe impl UITraitEnvironment for UIDocumentViewController {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIDocumentViewController {
        #[cfg(feature = "UIDocument")]
        #[method_id(@__retain_semantics Init initWithDocument:)]
        pub unsafe fn initWithDocument(
            this: Allocated<Self>,
            document: Option<&UIDocument>,
        ) -> Retained<Self>;

        #[cfg(feature = "UIDocument")]
        #[method_id(@__retain_semantics Other document)]
        pub unsafe fn document(&self) -> Option<Retained<UIDocument>>;

        #[cfg(feature = "UIDocument")]
        #[method(setDocument:)]
        pub unsafe fn setDocument(&self, document: Option<&UIDocument>);

        #[cfg(feature = "UIDocumentViewControllerLaunchOptions")]
        #[method_id(@__retain_semantics Other launchOptions)]
        pub unsafe fn launchOptions(&self) -> Retained<UIDocumentViewControllerLaunchOptions>;

        #[cfg(feature = "UIDocumentViewControllerLaunchOptions")]
        #[method(setLaunchOptions:)]
        pub unsafe fn setLaunchOptions(
            &self,
            launch_options: &UIDocumentViewControllerLaunchOptions,
        );

        #[method(navigationItemDidUpdate)]
        pub unsafe fn navigationItemDidUpdate(&self);

        #[cfg(feature = "block2")]
        #[method(openDocumentWithCompletionHandler:)]
        pub unsafe fn openDocumentWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(Bool)>,
        );

        #[method(documentDidOpen)]
        pub unsafe fn documentDidOpen(&self);

        #[cfg(feature = "UIBarButtonItemGroup")]
        #[method_id(@__retain_semantics Other undoRedoItemGroup)]
        pub unsafe fn undoRedoItemGroup(&self) -> Retained<UIBarButtonItemGroup>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIViewController`
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIDocumentViewController {
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
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIDocumentViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
