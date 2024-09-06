//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPDFPanelOptions(pub NSInteger);
bitflags::bitflags! {
    impl NSPDFPanelOptions: NSInteger {
        const NSPDFPanelShowsPaperSize = 1<<2;
        const NSPDFPanelShowsOrientation = 1<<3;
        const NSPDFPanelRequestsParentDirectory = 1<<24;
    }
}

unsafe impl Encode for NSPDFPanelOptions {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSPDFPanelOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPDFPanel;

    unsafe impl ClassType for NSPDFPanel {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for NSPDFPanel {}

extern_methods!(
    unsafe impl NSPDFPanel {
        #[method_id(@__retain_semantics Other panel)]
        pub unsafe fn panel(mtm: MainThreadMarker) -> Retained<NSPDFPanel>;

        #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
        #[method_id(@__retain_semantics Other accessoryController)]
        pub unsafe fn accessoryController(&self) -> Option<Retained<NSViewController>>;

        #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
        #[method(setAccessoryController:)]
        pub unsafe fn setAccessoryController(
            &self,
            accessory_controller: Option<&NSViewController>,
        );

        #[method(options)]
        pub unsafe fn options(&self) -> NSPDFPanelOptions;

        #[method(setOptions:)]
        pub unsafe fn setOptions(&self, options: NSPDFPanelOptions);

        #[method_id(@__retain_semantics Other defaultFileName)]
        pub unsafe fn defaultFileName(&self) -> Retained<NSString>;

        #[method(setDefaultFileName:)]
        pub unsafe fn setDefaultFileName(&self, default_file_name: &NSString);

        #[cfg(all(
            feature = "NSPDFInfo",
            feature = "NSResponder",
            feature = "NSWindow",
            feature = "block2"
        ))]
        #[method(beginSheetWithPDFInfo:modalForWindow:completionHandler:)]
        pub unsafe fn beginSheetWithPDFInfo_modalForWindow_completionHandler(
            &self,
            pdf_info: &NSPDFInfo,
            doc_window: Option<&NSWindow>,
            completion_handler: &block2::Block<dyn Fn(NSInteger)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPDFPanel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
