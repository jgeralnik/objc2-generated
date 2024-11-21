//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[unsafe(super(UIWindowSceneGeometryPreferences, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIWindowSceneGeometryPreferences")]
    pub struct UIWindowSceneGeometryPreferencesIOS;
);

#[cfg(feature = "UIWindowSceneGeometryPreferences")]
unsafe impl NSObjectProtocol for UIWindowSceneGeometryPreferencesIOS {}

extern_methods!(
    #[cfg(feature = "UIWindowSceneGeometryPreferences")]
    unsafe impl UIWindowSceneGeometryPreferencesIOS {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "UIOrientation")]
        #[method_id(@__retain_semantics Init initWithInterfaceOrientations:)]
        pub unsafe fn initWithInterfaceOrientations(
            this: Allocated<Self>,
            interface_orientations: UIInterfaceOrientationMask,
        ) -> Retained<Self>;

        #[cfg(feature = "UIOrientation")]
        #[method(interfaceOrientations)]
        pub unsafe fn interfaceOrientations(&self) -> UIInterfaceOrientationMask;

        #[cfg(feature = "UIOrientation")]
        #[method(setInterfaceOrientations:)]
        pub unsafe fn setInterfaceOrientations(
            &self,
            interface_orientations: UIInterfaceOrientationMask,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `UIWindowSceneGeometryPreferences`
    #[cfg(feature = "UIWindowSceneGeometryPreferences")]
    unsafe impl UIWindowSceneGeometryPreferencesIOS {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
