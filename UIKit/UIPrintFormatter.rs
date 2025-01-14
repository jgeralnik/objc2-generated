//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiprintformatter?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPrintFormatter;
);

unsafe impl NSCopying for UIPrintFormatter {}

unsafe impl CopyingHelper for UIPrintFormatter {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIPrintFormatter {}

extern_methods!(
    unsafe impl UIPrintFormatter {
        #[cfg(feature = "UIPrintPageRenderer")]
        #[method_id(@__retain_semantics Other printPageRenderer)]
        pub unsafe fn printPageRenderer(&self) -> Option<Retained<UIPrintPageRenderer>>;

        #[method(removeFromPrintPageRenderer)]
        pub unsafe fn removeFromPrintPageRenderer(&self);

        #[method(maximumContentHeight)]
        pub unsafe fn maximumContentHeight(&self) -> CGFloat;

        #[method(setMaximumContentHeight:)]
        pub unsafe fn setMaximumContentHeight(&self, maximum_content_height: CGFloat);

        #[method(maximumContentWidth)]
        pub unsafe fn maximumContentWidth(&self) -> CGFloat;

        #[method(setMaximumContentWidth:)]
        pub unsafe fn setMaximumContentWidth(&self, maximum_content_width: CGFloat);

        #[cfg(feature = "UIGeometry")]
        #[deprecated]
        #[method(contentInsets)]
        pub unsafe fn contentInsets(&self) -> UIEdgeInsets;

        #[cfg(feature = "UIGeometry")]
        #[deprecated]
        #[method(setContentInsets:)]
        pub unsafe fn setContentInsets(&self, content_insets: UIEdgeInsets);

        #[cfg(feature = "UIGeometry")]
        #[method(perPageContentInsets)]
        pub unsafe fn perPageContentInsets(&self) -> UIEdgeInsets;

        #[cfg(feature = "UIGeometry")]
        #[method(setPerPageContentInsets:)]
        pub unsafe fn setPerPageContentInsets(&self, per_page_content_insets: UIEdgeInsets);

        #[method(startPage)]
        pub unsafe fn startPage(&self) -> NSInteger;

        #[method(setStartPage:)]
        pub unsafe fn setStartPage(&self, start_page: NSInteger);

        #[method(pageCount)]
        pub unsafe fn pageCount(&self) -> NSInteger;

        #[method(requiresMainThread)]
        pub unsafe fn requiresMainThread(&self) -> bool;

        #[method(rectForPageAtIndex:)]
        pub unsafe fn rectForPageAtIndex(&self, page_index: NSInteger) -> CGRect;

        #[method(drawInRect:forPageAtIndex:)]
        pub unsafe fn drawInRect_forPageAtIndex(&self, rect: CGRect, page_index: NSInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIPrintFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uisimpletextprintformatter?language=objc)
    #[unsafe(super(UIPrintFormatter, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UISimpleTextPrintFormatter;
);

unsafe impl NSCopying for UISimpleTextPrintFormatter {}

unsafe impl CopyingHelper for UISimpleTextPrintFormatter {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UISimpleTextPrintFormatter {}

extern_methods!(
    unsafe impl UISimpleTextPrintFormatter {
        #[method_id(@__retain_semantics Init initWithText:)]
        pub unsafe fn initWithText(this: Allocated<Self>, text: &NSString) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithAttributedText:)]
        pub unsafe fn initWithAttributedText(
            this: Allocated<Self>,
            attributed_text: &NSAttributedString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other text)]
        pub unsafe fn text(&self) -> Option<Retained<NSString>>;

        #[method(setText:)]
        pub unsafe fn setText(&self, text: Option<&NSString>);

        #[method_id(@__retain_semantics Other attributedText)]
        pub unsafe fn attributedText(&self) -> Option<Retained<NSAttributedString>>;

        #[method(setAttributedText:)]
        pub unsafe fn setAttributedText(&self, attributed_text: Option<&NSAttributedString>);

        #[cfg(feature = "UIFont")]
        #[method_id(@__retain_semantics Other font)]
        pub unsafe fn font(&self) -> Option<Retained<UIFont>>;

        #[cfg(feature = "UIFont")]
        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: Option<&UIFont>);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other color)]
        pub unsafe fn color(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: Option<&UIColor>);

        #[cfg(feature = "NSText")]
        #[method(textAlignment)]
        pub unsafe fn textAlignment(&self) -> NSTextAlignment;

        #[cfg(feature = "NSText")]
        #[method(setTextAlignment:)]
        pub unsafe fn setTextAlignment(&self, text_alignment: NSTextAlignment);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UISimpleTextPrintFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uimarkuptextprintformatter?language=objc)
    #[unsafe(super(UIPrintFormatter, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIMarkupTextPrintFormatter;
);

unsafe impl NSCopying for UIMarkupTextPrintFormatter {}

unsafe impl CopyingHelper for UIMarkupTextPrintFormatter {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIMarkupTextPrintFormatter {}

extern_methods!(
    unsafe impl UIMarkupTextPrintFormatter {
        #[method_id(@__retain_semantics Init initWithMarkupText:)]
        pub unsafe fn initWithMarkupText(
            this: Allocated<Self>,
            markup_text: &NSString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other markupText)]
        pub unsafe fn markupText(&self) -> Option<Retained<NSString>>;

        #[method(setMarkupText:)]
        pub unsafe fn setMarkupText(&self, markup_text: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIMarkupTextPrintFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiviewprintformatter?language=objc)
    #[unsafe(super(UIPrintFormatter, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIViewPrintFormatter;
);

unsafe impl NSCopying for UIViewPrintFormatter {}

unsafe impl CopyingHelper for UIViewPrintFormatter {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIViewPrintFormatter {}

extern_methods!(
    unsafe impl UIViewPrintFormatter {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Retained<UIView>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIViewPrintFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// UIPrintFormatter
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIView {
        #[method_id(@__retain_semantics Other viewPrintFormatter)]
        pub unsafe fn viewPrintFormatter(&self) -> Retained<UIViewPrintFormatter>;

        #[method(drawRect:forViewPrintFormatter:)]
        pub unsafe fn drawRect_forViewPrintFormatter(
            &self,
            rect: CGRect,
            formatter: &UIViewPrintFormatter,
        );
    }
);
