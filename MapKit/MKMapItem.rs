//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKMapItem;
);

unsafe impl NSObjectProtocol for MKMapItem {}

extern_methods!(
    unsafe impl MKMapItem {
        #[cfg(feature = "MKMapItemIdentifier")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Retained<MKMapItemIdentifier>>;

        #[cfg(feature = "MKMapItemIdentifier")]
        #[method_id(@__retain_semantics Other alternateIdentifiers)]
        pub unsafe fn alternateIdentifiers(&self) -> Retained<NSSet<MKMapItemIdentifier>>;

        #[cfg(all(feature = "MKPlacemark", feature = "objc2-core-location"))]
        #[method_id(@__retain_semantics Other placemark)]
        pub unsafe fn placemark(&self) -> Retained<MKPlacemark>;

        #[method(isCurrentLocation)]
        pub unsafe fn isCurrentLocation(&self) -> bool;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Retained<NSString>>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[method_id(@__retain_semantics Other phoneNumber)]
        pub unsafe fn phoneNumber(&self) -> Option<Retained<NSString>>;

        #[method(setPhoneNumber:)]
        pub unsafe fn setPhoneNumber(&self, phone_number: Option<&NSString>);

        #[method_id(@__retain_semantics Other url)]
        pub unsafe fn url(&self) -> Option<Retained<NSURL>>;

        #[method(setUrl:)]
        pub unsafe fn setUrl(&self, url: Option<&NSURL>);

        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Option<Retained<NSTimeZone>>;

        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, time_zone: Option<&NSTimeZone>);

        #[cfg(feature = "MKPointOfInterestCategory")]
        #[method_id(@__retain_semantics Other pointOfInterestCategory)]
        pub unsafe fn pointOfInterestCategory(&self)
            -> Option<Retained<MKPointOfInterestCategory>>;

        #[cfg(feature = "MKPointOfInterestCategory")]
        #[method(setPointOfInterestCategory:)]
        pub unsafe fn setPointOfInterestCategory(
            &self,
            point_of_interest_category: Option<&MKPointOfInterestCategory>,
        );

        #[method_id(@__retain_semantics Other mapItemForCurrentLocation)]
        pub unsafe fn mapItemForCurrentLocation() -> Retained<MKMapItem>;

        #[cfg(all(feature = "MKPlacemark", feature = "objc2-core-location"))]
        #[method_id(@__retain_semantics Init initWithPlacemark:)]
        pub unsafe fn initWithPlacemark(
            this: Allocated<Self>,
            placemark: &MKPlacemark,
        ) -> Retained<Self>;

        #[method(openInMapsWithLaunchOptions:)]
        pub unsafe fn openInMapsWithLaunchOptions(
            &self,
            launch_options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> bool;

        #[method(openMapsWithItems:launchOptions:)]
        pub unsafe fn openMapsWithItems_launchOptions(
            map_items: &NSArray<MKMapItem>,
            launch_options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> bool;

        #[cfg(feature = "block2")]
        #[method(openInMapsWithLaunchOptions:completionHandler:)]
        pub unsafe fn openInMapsWithLaunchOptions_completionHandler(
            &self,
            launch_options: Option<&NSDictionary<NSString, AnyObject>>,
            completion: Option<&block2::Block<dyn Fn(Bool)>>,
        );

        #[cfg(feature = "block2")]
        #[method(openMapsWithItems:launchOptions:completionHandler:)]
        pub unsafe fn openMapsWithItems_launchOptions_completionHandler(
            map_items: &NSArray<MKMapItem>,
            launch_options: Option<&NSDictionary<NSString, AnyObject>>,
            completion: Option<&block2::Block<dyn Fn(Bool)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKMapItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    pub static MKLaunchOptionsDirectionsModeKey: &'static NSString;
}

extern "C" {
    pub static MKLaunchOptionsMapTypeKey: &'static NSString;
}

extern "C" {
    pub static MKLaunchOptionsShowsTrafficKey: &'static NSString;
}

extern "C" {
    pub static MKLaunchOptionsDirectionsModeDefault: &'static NSString;
}

extern "C" {
    pub static MKLaunchOptionsDirectionsModeDriving: &'static NSString;
}

extern "C" {
    pub static MKLaunchOptionsDirectionsModeWalking: &'static NSString;
}

extern "C" {
    pub static MKLaunchOptionsDirectionsModeTransit: &'static NSString;
}

extern "C" {
    pub static MKLaunchOptionsMapCenterKey: &'static NSString;
}

extern "C" {
    pub static MKLaunchOptionsMapSpanKey: &'static NSString;
}

extern "C" {
    pub static MKLaunchOptionsCameraKey: &'static NSString;
}

extern_methods!(
    /// MKMapItemSerialization
    unsafe impl MKMapItem {}
);

unsafe impl NSSecureCoding for MKMapItem {}

extern "C" {
    pub static MKMapItemTypeIdentifier: &'static NSString;
}
