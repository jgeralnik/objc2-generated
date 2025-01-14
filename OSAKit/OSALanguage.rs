//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/osakit/osalanguagefeatures?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OSALanguageFeatures(pub NSUInteger);
bitflags::bitflags! {
    impl OSALanguageFeatures: NSUInteger {
        const OSASupportsCompiling = 0x0002;
        const OSASupportsGetSource = 0x0004;
        const OSASupportsAECoercion = 0x0008;
        const OSASupportsAESending = 0x0010;
        const OSASupportsRecording = 0x0020;
        const OSASupportsConvenience = 0x0040;
        const OSASupportsDialects = 0x0080;
        const OSASupportsEventHandling = 0x0100;
    }
}

unsafe impl Encode for OSALanguageFeatures {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for OSALanguageFeatures {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/osakit/osalanguage?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct OSALanguage;
);

unsafe impl NSObjectProtocol for OSALanguage {}

extern_methods!(
    unsafe impl OSALanguage {
        #[method_id(@__retain_semantics Other availableLanguages)]
        pub unsafe fn availableLanguages() -> Retained<NSArray<OSALanguage>>;

        #[method_id(@__retain_semantics Other languageForName:)]
        pub unsafe fn languageForName(name: &NSString) -> Option<Retained<OSALanguage>>;

        #[method_id(@__retain_semantics Other languageForScriptDataDescriptor:)]
        pub unsafe fn languageForScriptDataDescriptor(
            descriptor: &NSAppleEventDescriptor,
        ) -> Option<Retained<OSALanguage>>;

        #[method_id(@__retain_semantics Other defaultLanguage)]
        pub unsafe fn defaultLanguage() -> Option<Retained<OSALanguage>>;

        #[method(setDefaultLanguage:)]
        pub unsafe fn setDefaultLanguage(default_language: &OSALanguage);

        #[cfg(feature = "OSALanguageInstance")]
        #[method_id(@__retain_semantics Other sharedLanguageInstance)]
        pub unsafe fn sharedLanguageInstance(&self) -> Retained<OSALanguageInstance>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other info)]
        pub unsafe fn info(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other version)]
        pub unsafe fn version(&self) -> Option<Retained<NSString>>;

        #[method(type)]
        pub unsafe fn r#type(&self) -> OSType;

        #[method(subType)]
        pub unsafe fn subType(&self) -> OSType;

        #[method(manufacturer)]
        pub unsafe fn manufacturer(&self) -> OSType;

        #[method(features)]
        pub unsafe fn features(&self) -> OSALanguageFeatures;

        #[method(isThreadSafe)]
        pub unsafe fn isThreadSafe(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl OSALanguage {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
