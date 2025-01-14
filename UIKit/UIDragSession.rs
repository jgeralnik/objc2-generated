//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidragdropsession?language=objc)
    pub unsafe trait UIDragDropSession: NSObjectProtocol + MainThreadOnly {
        #[cfg(feature = "UIDragItem")]
        #[method_id(@__retain_semantics Other items)]
        unsafe fn items(&self) -> Retained<NSArray<UIDragItem>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method(locationInView:)]
        unsafe fn locationInView(&self, view: &UIView) -> CGPoint;

        #[method(allowsMoveOperation)]
        unsafe fn allowsMoveOperation(&self) -> bool;

        #[method(isRestrictedToDraggingApplication)]
        unsafe fn isRestrictedToDraggingApplication(&self) -> bool;

        #[method(hasItemsConformingToTypeIdentifiers:)]
        unsafe fn hasItemsConformingToTypeIdentifiers(
            &self,
            type_identifiers: &NSArray<NSString>,
        ) -> bool;

        #[method(canLoadObjectsOfClass:)]
        unsafe fn canLoadObjectsOfClass(&self, a_class: &AnyClass) -> bool;
    }

    unsafe impl ProtocolType for dyn UIDragDropSession {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidragsession?language=objc)
    pub unsafe trait UIDragSession: UIDragDropSession + MainThreadOnly {
        #[method_id(@__retain_semantics Other localContext)]
        unsafe fn localContext(&self) -> Option<Retained<AnyObject>>;

        #[method(setLocalContext:)]
        unsafe fn setLocalContext(&self, local_context: Option<&AnyObject>);
    }

    unsafe impl ProtocolType for dyn UIDragSession {}
);

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidropsessionprogressindicatorstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIDropSessionProgressIndicatorStyle(pub NSUInteger);
impl UIDropSessionProgressIndicatorStyle {
    #[doc(alias = "UIDropSessionProgressIndicatorStyleNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "UIDropSessionProgressIndicatorStyleDefault")]
    pub const Default: Self = Self(1);
}

unsafe impl Encode for UIDropSessionProgressIndicatorStyle {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIDropSessionProgressIndicatorStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidropsession?language=objc)
    pub unsafe trait UIDropSession:
        NSProgressReporting + UIDragDropSession + MainThreadOnly
    {
        #[method_id(@__retain_semantics Other localDragSession)]
        unsafe fn localDragSession(&self) -> Option<Retained<ProtocolObject<dyn UIDragSession>>>;

        #[method(progressIndicatorStyle)]
        unsafe fn progressIndicatorStyle(&self) -> UIDropSessionProgressIndicatorStyle;

        #[method(setProgressIndicatorStyle:)]
        unsafe fn setProgressIndicatorStyle(
            &self,
            progress_indicator_style: UIDropSessionProgressIndicatorStyle,
        );

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other loadObjectsOfClass:completion:)]
        unsafe fn loadObjectsOfClass_completion(
            &self,
            a_class: &AnyClass,
            completion: &block2::Block<
                dyn Fn(NonNull<NSArray<ProtocolObject<dyn NSItemProviderReading>>>),
            >,
        ) -> Retained<NSProgress>;
    }

    unsafe impl ProtocolType for dyn UIDropSession {}
);
