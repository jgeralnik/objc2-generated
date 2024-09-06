//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPrintingPageOrder(pub NSInteger);
impl NSPrintingPageOrder {
    pub const NSDescendingPageOrder: Self = Self(-1);
    pub const NSSpecialPageOrder: Self = Self(0);
    pub const NSAscendingPageOrder: Self = Self(1);
    pub const NSUnknownPageOrder: Self = Self(2);
}

unsafe impl Encode for NSPrintingPageOrder {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSPrintingPageOrder {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPrintRenderingQuality(pub NSInteger);
impl NSPrintRenderingQuality {
    #[doc(alias = "NSPrintRenderingQualityBest")]
    pub const Best: Self = Self(0);
    #[doc(alias = "NSPrintRenderingQualityResponsive")]
    pub const Responsive: Self = Self(1);
}

unsafe impl Encode for NSPrintRenderingQuality {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSPrintRenderingQuality {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    pub static NSPrintOperationExistsException: &'static NSExceptionName;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPrintOperation;

    unsafe impl ClassType for NSPrintOperation {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for NSPrintOperation {}

extern_methods!(
    unsafe impl NSPrintOperation {
        #[cfg(all(feature = "NSPrintInfo", feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other printOperationWithView:printInfo:)]
        pub unsafe fn printOperationWithView_printInfo(
            view: &NSView,
            print_info: &NSPrintInfo,
        ) -> Retained<NSPrintOperation>;

        #[cfg(all(feature = "NSPrintInfo", feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other PDFOperationWithView:insideRect:toData:printInfo:)]
        pub unsafe fn PDFOperationWithView_insideRect_toData_printInfo(
            view: &NSView,
            rect: NSRect,
            data: &NSMutableData,
            print_info: &NSPrintInfo,
        ) -> Retained<NSPrintOperation>;

        #[cfg(all(feature = "NSPrintInfo", feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other PDFOperationWithView:insideRect:toPath:printInfo:)]
        pub unsafe fn PDFOperationWithView_insideRect_toPath_printInfo(
            view: &NSView,
            rect: NSRect,
            path: &NSString,
            print_info: &NSPrintInfo,
        ) -> Retained<NSPrintOperation>;

        #[cfg(all(feature = "NSPrintInfo", feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other EPSOperationWithView:insideRect:toData:printInfo:)]
        pub unsafe fn EPSOperationWithView_insideRect_toData_printInfo(
            view: &NSView,
            rect: NSRect,
            data: &NSMutableData,
            print_info: &NSPrintInfo,
        ) -> Retained<NSPrintOperation>;

        #[cfg(all(feature = "NSPrintInfo", feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other EPSOperationWithView:insideRect:toPath:printInfo:)]
        pub unsafe fn EPSOperationWithView_insideRect_toPath_printInfo(
            view: &NSView,
            rect: NSRect,
            path: &NSString,
            print_info: &NSPrintInfo,
        ) -> Retained<NSPrintOperation>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other printOperationWithView:)]
        pub unsafe fn printOperationWithView(view: &NSView) -> Retained<NSPrintOperation>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other PDFOperationWithView:insideRect:toData:)]
        pub unsafe fn PDFOperationWithView_insideRect_toData(
            view: &NSView,
            rect: NSRect,
            data: &NSMutableData,
        ) -> Retained<NSPrintOperation>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other EPSOperationWithView:insideRect:toData:)]
        pub unsafe fn EPSOperationWithView_insideRect_toData(
            view: &NSView,
            rect: NSRect,
            data: Option<&NSMutableData>,
        ) -> Retained<NSPrintOperation>;

        #[method_id(@__retain_semantics Other currentOperation)]
        pub unsafe fn currentOperation(mtm: MainThreadMarker)
            -> Option<Retained<NSPrintOperation>>;

        #[method(setCurrentOperation:)]
        pub unsafe fn setCurrentOperation(
            current_operation: Option<&NSPrintOperation>,
            mtm: MainThreadMarker,
        );

        #[method(isCopyingOperation)]
        pub unsafe fn isCopyingOperation(&self) -> bool;

        #[method(preferredRenderingQuality)]
        pub unsafe fn preferredRenderingQuality(&self) -> NSPrintRenderingQuality;

        #[method_id(@__retain_semantics Other jobTitle)]
        pub unsafe fn jobTitle(&self) -> Option<Retained<NSString>>;

        #[method(setJobTitle:)]
        pub unsafe fn setJobTitle(&self, job_title: Option<&NSString>);

        #[method(showsPrintPanel)]
        pub unsafe fn showsPrintPanel(&self) -> bool;

        #[method(setShowsPrintPanel:)]
        pub unsafe fn setShowsPrintPanel(&self, shows_print_panel: bool);

        #[method(showsProgressPanel)]
        pub unsafe fn showsProgressPanel(&self) -> bool;

        #[method(setShowsProgressPanel:)]
        pub unsafe fn setShowsProgressPanel(&self, shows_progress_panel: bool);

        #[cfg(feature = "NSPrintPanel")]
        #[method_id(@__retain_semantics Other printPanel)]
        pub unsafe fn printPanel(&self) -> Retained<NSPrintPanel>;

        #[cfg(feature = "NSPrintPanel")]
        #[method(setPrintPanel:)]
        pub unsafe fn setPrintPanel(&self, print_panel: &NSPrintPanel);

        #[cfg(feature = "NSPDFPanel")]
        #[method_id(@__retain_semantics Other PDFPanel)]
        pub unsafe fn PDFPanel(&self) -> Retained<NSPDFPanel>;

        #[cfg(feature = "NSPDFPanel")]
        #[method(setPDFPanel:)]
        pub unsafe fn setPDFPanel(&self, pdf_panel: &NSPDFPanel);

        #[method(canSpawnSeparateThread)]
        pub unsafe fn canSpawnSeparateThread(&self) -> bool;

        #[method(setCanSpawnSeparateThread:)]
        pub unsafe fn setCanSpawnSeparateThread(&self, can_spawn_separate_thread: bool);

        #[method(pageOrder)]
        pub unsafe fn pageOrder(&self) -> NSPrintingPageOrder;

        #[method(setPageOrder:)]
        pub unsafe fn setPageOrder(&self, page_order: NSPrintingPageOrder);

        #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
        #[method(runOperationModalForWindow:delegate:didRunSelector:contextInfo:)]
        pub unsafe fn runOperationModalForWindow_delegate_didRunSelector_contextInfo(
            &self,
            doc_window: &NSWindow,
            delegate: Option<&AnyObject>,
            did_run_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[method(runOperation)]
        pub unsafe fn runOperation(&self) -> bool;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Retained<NSView>>;

        #[cfg(feature = "NSPrintInfo")]
        #[method_id(@__retain_semantics Other printInfo)]
        pub unsafe fn printInfo(&self) -> Retained<NSPrintInfo>;

        #[cfg(feature = "NSPrintInfo")]
        #[method(setPrintInfo:)]
        pub unsafe fn setPrintInfo(&self, print_info: &NSPrintInfo);

        #[cfg(feature = "NSGraphicsContext")]
        #[method_id(@__retain_semantics Other context)]
        pub unsafe fn context(&self) -> Option<Retained<NSGraphicsContext>>;

        #[method(pageRange)]
        pub unsafe fn pageRange(&self) -> NSRange;

        #[method(currentPage)]
        pub unsafe fn currentPage(&self) -> NSInteger;

        #[cfg(feature = "NSGraphicsContext")]
        #[method_id(@__retain_semantics Other createContext)]
        pub unsafe fn createContext(&self) -> Option<Retained<NSGraphicsContext>>;

        #[method(destroyContext)]
        pub unsafe fn destroyContext(&self);

        #[method(deliverResult)]
        pub unsafe fn deliverResult(&self) -> bool;

        #[method(cleanUpOperation)]
        pub unsafe fn cleanUpOperation(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPrintOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSPrintOperation {
        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[deprecated = "Use -[NSPrintPanel addAccessoryController:] and -[NSPrintPanel removeAccessoryController:] instead"]
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, view: Option<&NSView>);

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[deprecated = "Use -[NSPrintPanel accessoryControllers] instead"]
        #[method_id(@__retain_semantics Other accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Retained<NSView>>;

        #[deprecated]
        #[method(setJobStyleHint:)]
        pub unsafe fn setJobStyleHint(&self, hint: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other jobStyleHint)]
        pub unsafe fn jobStyleHint(&self) -> Option<Retained<NSString>>;

        #[deprecated = "Use -setShowsPrintPanel: and -setShowsProgressPanel: instead"]
        #[method(setShowPanels:)]
        pub unsafe fn setShowPanels(&self, flag: bool);

        #[deprecated = "Use -showsPrintPanel and -showsProgressPanel instead"]
        #[method(showPanels)]
        pub unsafe fn showPanels(&self) -> bool;
    }
);
