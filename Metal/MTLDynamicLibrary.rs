//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static MTLDynamicLibraryDomain: &'static NSErrorDomain;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLDynamicLibraryError(pub NSUInteger);
impl MTLDynamicLibraryError {
    #[doc(alias = "MTLDynamicLibraryErrorNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "MTLDynamicLibraryErrorInvalidFile")]
    pub const InvalidFile: Self = Self(1);
    #[doc(alias = "MTLDynamicLibraryErrorCompilationFailure")]
    pub const CompilationFailure: Self = Self(2);
    #[doc(alias = "MTLDynamicLibraryErrorUnresolvedInstallName")]
    pub const UnresolvedInstallName: Self = Self(3);
    #[doc(alias = "MTLDynamicLibraryErrorDependencyLoadFailure")]
    pub const DependencyLoadFailure: Self = Self(4);
    #[doc(alias = "MTLDynamicLibraryErrorUnsupported")]
    pub const Unsupported: Self = Self(5);
}

unsafe impl Encode for MTLDynamicLibraryError {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLDynamicLibraryError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait MTLDynamicLibrary: NSObjectProtocol {
        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Retained<NSString>>;

        #[method(setLabel:)]
        fn setLabel(&self, label: Option<&NSString>);

        #[cfg(feature = "MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        #[method_id(@__retain_semantics Other installName)]
        fn installName(&self) -> Retained<NSString>;

        #[method(serializeToURL:error:_)]
        fn serializeToURL_error(&self, url: &NSURL) -> Result<(), Retained<NSError>>;
    }

    unsafe impl ProtocolType for dyn MTLDynamicLibrary {}
);
