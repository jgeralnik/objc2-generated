//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTableCellView;

    unsafe impl ClassType for NSTableCellView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTableCellView")]
    unsafe impl NSTableCellView {
        #[method_id(@__retain_semantics Other objectValue)]
        pub unsafe fn objectValue(&self) -> Option<Id<Object, Shared>>;

        #[method(setObjectValue:)]
        pub unsafe fn setObjectValue(&self, objectValue: Option<&Object>);

        #[cfg(feature = "AppKit_NSTextField")]
        #[method_id(@__retain_semantics Other textField)]
        pub unsafe fn textField(&self) -> Option<Id<NSTextField, Shared>>;

        #[cfg(feature = "AppKit_NSTextField")]
        #[method(setTextField:)]
        pub unsafe fn setTextField(&self, textField: Option<&NSTextField>);

        #[cfg(feature = "AppKit_NSImageView")]
        #[method_id(@__retain_semantics Other imageView)]
        pub unsafe fn imageView(&self) -> Option<Id<NSImageView, Shared>>;

        #[cfg(feature = "AppKit_NSImageView")]
        #[method(setImageView:)]
        pub unsafe fn setImageView(&self, imageView: Option<&NSImageView>);

        #[method(backgroundStyle)]
        pub unsafe fn backgroundStyle(&self) -> NSBackgroundStyle;

        #[method(setBackgroundStyle:)]
        pub unsafe fn setBackgroundStyle(&self, backgroundStyle: NSBackgroundStyle);

        #[method(rowSizeStyle)]
        pub unsafe fn rowSizeStyle(&self) -> NSTableViewRowSizeStyle;

        #[method(setRowSizeStyle:)]
        pub unsafe fn setRowSizeStyle(&self, rowSizeStyle: NSTableViewRowSizeStyle);

        #[cfg(feature = "AppKit_NSDraggingImageComponent")]
        #[method_id(@__retain_semantics Other draggingImageComponents)]
        pub unsafe fn draggingImageComponents(
            &self,
        ) -> Id<NSArray<NSDraggingImageComponent>, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSTableCellView")]
    unsafe impl NSTableCellView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: NSRect,
        ) -> Id<Self, Shared>;
    }
);
