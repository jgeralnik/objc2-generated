//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Loading Interface Builder products will not be supported in a future version of visionOS."]
    pub struct UINib;

    unsafe impl ClassType for UINib {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UINib {}

extern_methods!(
    unsafe impl UINib {
        #[deprecated = "Loading Interface Builder products will not be supported in a future version of visionOS."]
        #[method_id(@__retain_semantics Other nibWithNibName:bundle:)]
        pub unsafe fn nibWithNibName_bundle(
            name: &NSString,
            bundle_or_nil: Option<&NSBundle>,
            mtm: MainThreadMarker,
        ) -> Retained<UINib>;

        #[deprecated = "Loading Interface Builder products will not be supported in a future version of visionOS."]
        #[method_id(@__retain_semantics Other nibWithData:bundle:)]
        pub unsafe fn nibWithData_bundle(
            data: &NSData,
            bundle_or_nil: Option<&NSBundle>,
            mtm: MainThreadMarker,
        ) -> Retained<UINib>;

        #[cfg(feature = "UINibLoading")]
        #[deprecated = "Loading Interface Builder products will not be supported in a future version of visionOS."]
        #[method_id(@__retain_semantics Other instantiateWithOwner:options:)]
        pub unsafe fn instantiateWithOwner_options(
            &self,
            owner_or_nil: Option<&AnyObject>,
            options_or_nil: Option<&NSDictionary<UINibOptionsKey, AnyObject>>,
        ) -> Retained<NSArray>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UINib {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
