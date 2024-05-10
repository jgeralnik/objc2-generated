//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-link-presentation")]
use objc2_link_presentation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait UIActivityItemSource: NSObjectProtocol {
        #[cfg(all(
            feature = "UIActivityViewController",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[method_id(@__retain_semantics Other activityViewControllerPlaceholderItem:)]
        unsafe fn activityViewControllerPlaceholderItem(
            &self,
            activity_view_controller: &UIActivityViewController,
        ) -> Id<AnyObject>;

        #[cfg(all(
            feature = "UIActivity",
            feature = "UIActivityViewController",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[method_id(@__retain_semantics Other activityViewController:itemForActivityType:)]
        unsafe fn activityViewController_itemForActivityType(
            &self,
            activity_view_controller: &UIActivityViewController,
            activity_type: Option<&UIActivityType>,
        ) -> Option<Id<AnyObject>>;

        #[cfg(all(
            feature = "UIActivity",
            feature = "UIActivityViewController",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other activityViewController:subjectForActivityType:)]
        unsafe fn activityViewController_subjectForActivityType(
            &self,
            activity_view_controller: &UIActivityViewController,
            activity_type: Option<&UIActivityType>,
        ) -> Id<NSString>;

        #[cfg(all(
            feature = "UIActivity",
            feature = "UIActivityViewController",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other activityViewController:dataTypeIdentifierForActivityType:)]
        unsafe fn activityViewController_dataTypeIdentifierForActivityType(
            &self,
            activity_view_controller: &UIActivityViewController,
            activity_type: Option<&UIActivityType>,
        ) -> Id<NSString>;

        #[cfg(all(
            feature = "UIActivity",
            feature = "UIActivityViewController",
            feature = "UIImage",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other activityViewController:thumbnailImageForActivityType:suggestedSize:)]
        unsafe fn activityViewController_thumbnailImageForActivityType_suggestedSize(
            &self,
            activity_view_controller: &UIActivityViewController,
            activity_type: Option<&UIActivityType>,
            size: CGSize,
        ) -> Option<Id<UIImage>>;

        #[cfg(all(
            feature = "UIActivityViewController",
            feature = "UIResponder",
            feature = "UIViewController",
            feature = "objc2-link-presentation"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other activityViewControllerLinkMetadata:)]
        unsafe fn activityViewControllerLinkMetadata(
            &self,
            activity_view_controller: &UIActivityViewController,
        ) -> Option<Id<LPLinkMetadata>>;
    }

    unsafe impl ProtocolType for dyn UIActivityItemSource {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIActivityItemProvider;

    unsafe impl ClassType for UIActivityItemProvider {
        #[inherits(NSObject)]
        type Super = NSOperation;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for UIActivityItemProvider {}

unsafe impl UIActivityItemSource for UIActivityItemProvider {}

extern_methods!(
    unsafe impl UIActivityItemProvider {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithPlaceholderItem:)]
        pub unsafe fn initWithPlaceholderItem(
            this: Allocated<Self>,
            placeholder_item: &AnyObject,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other placeholderItem)]
        pub unsafe fn placeholderItem(&self) -> Option<Id<AnyObject>>;

        #[cfg(feature = "UIActivity")]
        #[method_id(@__retain_semantics Other activityType)]
        pub unsafe fn activityType(&self) -> Option<Id<UIActivityType>>;

        #[method_id(@__retain_semantics Other item)]
        pub unsafe fn item(&self) -> Id<AnyObject>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIActivityItemProvider {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);