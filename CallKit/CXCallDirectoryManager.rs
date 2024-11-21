//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CXCallDirectoryEnabledStatus(pub NSInteger);
impl CXCallDirectoryEnabledStatus {
    #[doc(alias = "CXCallDirectoryEnabledStatusUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "CXCallDirectoryEnabledStatusDisabled")]
    pub const Disabled: Self = Self(1);
    #[doc(alias = "CXCallDirectoryEnabledStatusEnabled")]
    pub const Enabled: Self = Self(2);
}

unsafe impl Encode for CXCallDirectoryEnabledStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CXCallDirectoryEnabledStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CXCallDirectoryManager;
);

unsafe impl NSObjectProtocol for CXCallDirectoryManager {}

extern_methods!(
    unsafe impl CXCallDirectoryManager {
        #[method_id(@__retain_semantics Other sharedInstance)]
        pub unsafe fn sharedInstance() -> Retained<CXCallDirectoryManager>;

        #[cfg(feature = "block2")]
        #[method(reloadExtensionWithIdentifier:completionHandler:)]
        pub unsafe fn reloadExtensionWithIdentifier_completionHandler(
            &self,
            identifier: &NSString,
            completion: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(getEnabledStatusForExtensionWithIdentifier:completionHandler:)]
        pub unsafe fn getEnabledStatusForExtensionWithIdentifier_completionHandler(
            &self,
            identifier: &NSString,
            completion: &block2::Block<dyn Fn(CXCallDirectoryEnabledStatus, *mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(openSettingsWithCompletionHandler:)]
        pub unsafe fn openSettingsWithCompletionHandler(
            &self,
            completion: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CXCallDirectoryManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
