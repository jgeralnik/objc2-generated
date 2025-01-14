//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkselectionaccessory?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKSelectionAccessory;
);

unsafe impl NSObjectProtocol for MKSelectionAccessory {}

extern_methods!(
    unsafe impl MKSelectionAccessory {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other mapItemDetailWithPresentationStyle:)]
        pub unsafe fn mapItemDetailWithPresentationStyle(
            presentation_style: &MKMapItemDetailSelectionAccessoryPresentationStyle,
        ) -> Retained<MKSelectionAccessory>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkmapitemdetailselectionaccessorycalloutstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MKMapItemDetailSelectionAccessoryCalloutStyle(pub NSInteger);
impl MKMapItemDetailSelectionAccessoryCalloutStyle {
    #[doc(alias = "MKMapItemDetailSelectionAccessoryCalloutStyleAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "MKMapItemDetailSelectionAccessoryCalloutStyleFull")]
    pub const Full: Self = Self(1);
    #[doc(alias = "MKMapItemDetailSelectionAccessoryCalloutStyleCompact")]
    pub const Compact: Self = Self(2);
}

unsafe impl Encode for MKMapItemDetailSelectionAccessoryCalloutStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MKMapItemDetailSelectionAccessoryCalloutStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkmapitemdetailselectionaccessorypresentationstyle?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKMapItemDetailSelectionAccessoryPresentationStyle;
);

unsafe impl NSObjectProtocol for MKMapItemDetailSelectionAccessoryPresentationStyle {}

extern_methods!(
    unsafe impl MKMapItemDetailSelectionAccessoryPresentationStyle {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Other automaticWithPresentationViewController:)]
        pub unsafe fn automaticWithPresentationViewController(
            presentation_view_controller: Option<&NSViewController>,
        ) -> Retained<MKMapItemDetailSelectionAccessoryPresentationStyle>;

        #[method_id(@__retain_semantics Other callout)]
        pub unsafe fn callout() -> Retained<MKMapItemDetailSelectionAccessoryPresentationStyle>;

        #[method_id(@__retain_semantics Other calloutWithCalloutStyle:)]
        pub unsafe fn calloutWithCalloutStyle(
            style: MKMapItemDetailSelectionAccessoryCalloutStyle,
        ) -> Retained<MKMapItemDetailSelectionAccessoryPresentationStyle>;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Other sheetPresentedFromViewController:)]
        pub unsafe fn sheetPresentedFromViewController(
            view_controller: &NSViewController,
        ) -> Retained<MKMapItemDetailSelectionAccessoryPresentationStyle>;

        #[method_id(@__retain_semantics Other openInMaps)]
        pub unsafe fn openInMaps() -> Retained<MKMapItemDetailSelectionAccessoryPresentationStyle>;
    }
);
