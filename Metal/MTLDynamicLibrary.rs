//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern "C" {
    #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
    pub static MTLDynamicLibraryDomain: &'static NSErrorDomain;
}

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLDynamicLibraryError {
        #[doc(alias = "MTLDynamicLibraryErrorNone")]
        None = 0,
        #[doc(alias = "MTLDynamicLibraryErrorInvalidFile")]
        InvalidFile = 1,
        #[doc(alias = "MTLDynamicLibraryErrorCompilationFailure")]
        CompilationFailure = 2,
        #[doc(alias = "MTLDynamicLibraryErrorUnresolvedInstallName")]
        UnresolvedInstallName = 3,
        #[doc(alias = "MTLDynamicLibraryErrorDependencyLoadFailure")]
        DependencyLoadFailure = 4,
        #[doc(alias = "MTLDynamicLibraryErrorUnsupported")]
        Unsupported = 5,
    }
);

extern_protocol!(
    pub unsafe trait MTLDynamicLibrary: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        fn setLabel(&self, label: Option<&NSString>);

        #[cfg(feature = "Metal_MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other installName)]
        fn installName(&self) -> Id<NSString>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(serializeToURL:error:_)]
        fn serializeToURL_error(&self, url: &NSURL) -> Result<(), Id<NSError>>;
    }

    unsafe impl ProtocolType for dyn MTLDynamicLibrary {}
);
