//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UITab")]
    pub struct UISearchTab;

    #[cfg(feature = "UITab")]
    unsafe impl ClassType for UISearchTab {
        #[inherits(NSObject)]
        type Super = UITab;
        type ThreadKind = dyn MainThreadOnly;
    }
);

#[cfg(feature = "UITab")]
unsafe impl NSObjectProtocol for UISearchTab {}

extern_methods!(
    #[cfg(feature = "UITab")]
    unsafe impl UISearchTab {
        #[cfg(all(
            feature = "UIResponder",
            feature = "UIViewController",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Init initWithViewControllerProvider:)]
        pub unsafe fn initWithViewControllerProvider(
            this: Allocated<Self>,
            view_controller_provider: Option<
                &block2::Block<dyn Fn(NonNull<UITab>) -> NonNull<UIViewController>>,
            >,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UITab`
    #[cfg(feature = "UITab")]
    unsafe impl UISearchTab {
        #[cfg(all(
            feature = "UIImage",
            feature = "UIResponder",
            feature = "UIViewController",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Init initWithTitle:image:identifier:viewControllerProvider:)]
        pub unsafe fn initWithTitle_image_identifier_viewControllerProvider(
            this: Allocated<Self>,
            title: &NSString,
            image: Option<&UIImage>,
            identifier: &NSString,
            view_controller_provider: Option<
                &block2::Block<dyn Fn(NonNull<UITab>) -> NonNull<UIViewController>>,
            >,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);