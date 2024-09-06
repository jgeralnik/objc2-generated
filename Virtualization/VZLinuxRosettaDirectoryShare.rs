//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VZLinuxRosettaAvailability(pub NSInteger);
impl VZLinuxRosettaAvailability {
    #[doc(alias = "VZLinuxRosettaAvailabilityNotSupported")]
    pub const NotSupported: Self = Self(0);
    #[doc(alias = "VZLinuxRosettaAvailabilityNotInstalled")]
    pub const NotInstalled: Self = Self(1);
    #[doc(alias = "VZLinuxRosettaAvailabilityInstalled")]
    pub const Installed: Self = Self(2);
}

unsafe impl Encode for VZLinuxRosettaAvailability {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for VZLinuxRosettaAvailability {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZDirectoryShare")]
    pub struct VZLinuxRosettaDirectoryShare;

    #[cfg(feature = "VZDirectoryShare")]
    unsafe impl ClassType for VZLinuxRosettaDirectoryShare {
        #[inherits(NSObject)]
        type Super = VZDirectoryShare;
    }
);

#[cfg(feature = "VZDirectoryShare")]
unsafe impl NSObjectProtocol for VZLinuxRosettaDirectoryShare {}

extern_methods!(
    #[cfg(feature = "VZDirectoryShare")]
    unsafe impl VZLinuxRosettaDirectoryShare {
        #[method_id(@__retain_semantics Init initWithError:_)]
        pub unsafe fn initWithError(
            this: Allocated<Self>,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "block2")]
        #[method(installRosettaWithCompletionHandler:)]
        pub unsafe fn installRosettaWithCompletionHandler(
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "VZLinuxRosettaCachingOptions")]
        #[method_id(@__retain_semantics Other options)]
        pub unsafe fn options(&self) -> Option<Retained<VZLinuxRosettaCachingOptions>>;

        #[cfg(feature = "VZLinuxRosettaCachingOptions")]
        #[method(setOptions:)]
        pub unsafe fn setOptions(&self, options: Option<&VZLinuxRosettaCachingOptions>);

        #[method(availability)]
        pub unsafe fn availability() -> VZLinuxRosettaAvailability;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZDirectoryShare`
    #[cfg(feature = "VZDirectoryShare")]
    unsafe impl VZLinuxRosettaDirectoryShare {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
