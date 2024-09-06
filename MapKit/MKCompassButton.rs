//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    pub struct MKCompassButton;

    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl ClassType for MKCompassButton {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
    }
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSAccessibility for MKCompassButton {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSAccessibilityElementProtocol for MKCompassButton {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSAnimatablePropertyContainer for MKCompassButton {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSAppearanceCustomization for MKCompassButton {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSCoding for MKCompassButton {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSDraggingDestination for MKCompassButton {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for MKCompassButton {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSUserInterfaceItemIdentification for MKCompassButton {}

extern_methods!(
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl MKCompassButton {
        #[cfg(feature = "MKMapView")]
        #[method_id(@__retain_semantics Other compassButtonWithMapView:)]
        pub unsafe fn compassButtonWithMapView(
            map_view: Option<&MKMapView>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "MKMapView")]
        #[method_id(@__retain_semantics Other mapView)]
        pub unsafe fn mapView(&self) -> Option<Retained<MKMapView>>;

        #[cfg(feature = "MKMapView")]
        #[method(setMapView:)]
        pub unsafe fn setMapView(&self, map_view: Option<&MKMapView>);

        #[cfg(feature = "MKTypes")]
        #[method(compassVisibility)]
        pub unsafe fn compassVisibility(&self) -> MKFeatureVisibility;

        #[cfg(feature = "MKTypes")]
        #[method(setCompassVisibility:)]
        pub unsafe fn setCompassVisibility(&self, compass_visibility: MKFeatureVisibility);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl MKCompassButton {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl MKCompassButton {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl MKCompassButton {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
