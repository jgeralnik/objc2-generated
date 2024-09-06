//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static OSAScriptErrorMessageKey: &'static NSString;
}

extern "C" {
    pub static OSAScriptErrorBriefMessageKey: &'static NSString;
}

extern "C" {
    pub static OSAScriptErrorNumberKey: &'static NSString;
}

extern "C" {
    pub static OSAScriptErrorPartialResultKey: &'static NSString;
}

extern "C" {
    pub static OSAScriptErrorOffendingObjectKey: &'static NSString;
}

extern "C" {
    pub static OSAScriptErrorExpectedTypeKey: &'static NSString;
}

extern "C" {
    pub static OSAScriptErrorAppAddressKey: &'static NSString;
}

extern "C" {
    pub static OSAScriptErrorAppNameKey: &'static NSString;
}

extern "C" {
    pub static OSAScriptErrorRangeKey: &'static NSString;
}

extern "C" {
    pub static OSAScriptErrorMessage: &'static NSString;
}

extern "C" {
    pub static OSAScriptErrorNumber: &'static NSString;
}

extern "C" {
    pub static OSAScriptErrorAppName: &'static NSString;
}

extern "C" {
    pub static OSAScriptErrorBriefMessage: &'static NSString;
}

extern "C" {
    pub static OSAScriptErrorRange: &'static NSString;
}

extern "C" {
    pub static OSAStorageScriptType: &'static NSString;
}

extern "C" {
    pub static OSAStorageScriptBundleType: &'static NSString;
}

extern "C" {
    pub static OSAStorageApplicationType: &'static NSString;
}

extern "C" {
    pub static OSAStorageApplicationBundleType: &'static NSString;
}

extern "C" {
    pub static OSAStorageTextType: &'static NSString;
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OSAStorageOptions(pub NSUInteger);
bitflags::bitflags! {
    impl OSAStorageOptions: NSUInteger {
        const OSANull = 0x00000000;
        const OSAPreventGetSource = 0x00000001;
        const OSACompileIntoContext = 0x00000002;
        const OSADontSetScriptLocation = 0x01000000;
        const OSAStayOpenApplet = 0x10000000;
        const OSAShowStartupScreen = 0x20000000;
    }
}

unsafe impl Encode for OSAStorageOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for OSAStorageOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct OSAScript;

    unsafe impl ClassType for OSAScript {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for OSAScript {}

unsafe impl CopyingHelper for OSAScript {
    type Result = Self;
}

unsafe impl NSObjectProtocol for OSAScript {}

extern_methods!(
    unsafe impl OSAScript {
        #[method_id(@__retain_semantics Other scriptDataDescriptorWithContentsOfURL:)]
        pub unsafe fn scriptDataDescriptorWithContentsOfURL(
            url: &NSURL,
        ) -> Option<Retained<NSAppleEventDescriptor>>;

        #[method_id(@__retain_semantics Init initWithSource:)]
        pub unsafe fn initWithSource(this: Allocated<Self>, source: &NSString) -> Retained<Self>;

        #[cfg(feature = "OSALanguage")]
        #[method_id(@__retain_semantics Init initWithSource:language:)]
        pub unsafe fn initWithSource_language(
            this: Allocated<Self>,
            source: &NSString,
            language: Option<&OSALanguage>,
        ) -> Retained<Self>;

        #[cfg(feature = "OSALanguageInstance")]
        #[method_id(@__retain_semantics Init initWithSource:fromURL:languageInstance:usingStorageOptions:)]
        pub unsafe fn initWithSource_fromURL_languageInstance_usingStorageOptions(
            this: Allocated<Self>,
            source: &NSString,
            url: Option<&NSURL>,
            instance: Option<&OSALanguageInstance>,
            storage_options: OSAStorageOptions,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithContentsOfURL:error:)]
        pub unsafe fn initWithContentsOfURL_error(
            this: Allocated<Self>,
            url: &NSURL,
            error_info: Option<&mut Option<Retained<NSDictionary<NSString, AnyObject>>>>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "OSALanguage")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:language:error:)]
        pub unsafe fn initWithContentsOfURL_language_error(
            this: Allocated<Self>,
            url: &NSURL,
            language: &OSALanguage,
            error_info: Option<&mut Option<Retained<NSDictionary<NSString, AnyObject>>>>,
        ) -> Retained<Self>;

        #[cfg(feature = "OSALanguageInstance")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:languageInstance:usingStorageOptions:error:_)]
        pub unsafe fn initWithContentsOfURL_languageInstance_usingStorageOptions_error(
            this: Allocated<Self>,
            url: &NSURL,
            instance: Option<&OSALanguageInstance>,
            storage_options: OSAStorageOptions,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[deprecated]
        #[method_id(@__retain_semantics Init initWithCompiledData:error:)]
        pub unsafe fn initWithCompiledData_error(
            this: Allocated<Self>,
            data: &NSData,
            error_info: Option<&mut Option<Retained<NSDictionary<NSString, AnyObject>>>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCompiledData:fromURL:usingStorageOptions:error:_)]
        pub unsafe fn initWithCompiledData_fromURL_usingStorageOptions_error(
            this: Allocated<Self>,
            data: &NSData,
            url: Option<&NSURL>,
            storage_options: OSAStorageOptions,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "OSALanguageInstance")]
        #[method_id(@__retain_semantics Init initWithScriptDataDescriptor:fromURL:languageInstance:usingStorageOptions:error:_)]
        pub unsafe fn initWithScriptDataDescriptor_fromURL_languageInstance_usingStorageOptions_error(
            this: Allocated<Self>,
            data: &NSAppleEventDescriptor,
            url: Option<&NSURL>,
            instance: Option<&OSALanguageInstance>,
            storage_options: OSAStorageOptions,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method_id(@__retain_semantics Other source)]
        pub unsafe fn source(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other url)]
        pub unsafe fn url(&self) -> Option<Retained<NSURL>>;

        #[cfg(feature = "OSALanguage")]
        #[method_id(@__retain_semantics Other language)]
        pub unsafe fn language(&self) -> Retained<OSALanguage>;

        #[cfg(feature = "OSALanguage")]
        #[method(setLanguage:)]
        pub unsafe fn setLanguage(&self, language: &OSALanguage);

        #[cfg(feature = "OSALanguageInstance")]
        #[method_id(@__retain_semantics Other languageInstance)]
        pub unsafe fn languageInstance(&self) -> Retained<OSALanguageInstance>;

        #[cfg(feature = "OSALanguageInstance")]
        #[method(setLanguageInstance:)]
        pub unsafe fn setLanguageInstance(&self, language_instance: &OSALanguageInstance);

        #[method(isCompiled)]
        pub unsafe fn isCompiled(&self) -> bool;

        #[method(compileAndReturnError:)]
        pub unsafe fn compileAndReturnError(
            &self,
            error_info: Option<&mut Option<Retained<NSDictionary<NSString, AnyObject>>>>,
        ) -> bool;

        #[method_id(@__retain_semantics Other executeAndReturnError:)]
        pub unsafe fn executeAndReturnError(
            &self,
            error_info: Option<&mut Option<Retained<NSDictionary<NSString, AnyObject>>>>,
        ) -> Option<Retained<NSAppleEventDescriptor>>;

        #[method_id(@__retain_semantics Other executeAppleEvent:error:)]
        pub unsafe fn executeAppleEvent_error(
            &self,
            event: &NSAppleEventDescriptor,
            error_info: Option<&mut Option<Retained<NSDictionary<NSString, AnyObject>>>>,
        ) -> Option<Retained<NSAppleEventDescriptor>>;

        #[method_id(@__retain_semantics Other executeAndReturnDisplayValue:error:)]
        pub unsafe fn executeAndReturnDisplayValue_error(
            &self,
            display_value: &mut Option<Retained<NSAttributedString>>,
            error_info: Option<&mut Option<Retained<NSDictionary<NSString, AnyObject>>>>,
        ) -> Option<Retained<NSAppleEventDescriptor>>;

        #[method_id(@__retain_semantics Other executeHandlerWithName:arguments:error:)]
        pub unsafe fn executeHandlerWithName_arguments_error(
            &self,
            name: &NSString,
            arguments: &NSArray,
            error_info: Option<&mut Option<Retained<NSDictionary<NSString, AnyObject>>>>,
        ) -> Option<Retained<NSAppleEventDescriptor>>;

        #[method_id(@__retain_semantics Other richTextSource)]
        pub unsafe fn richTextSource(&self) -> Option<Retained<NSAttributedString>>;

        #[method_id(@__retain_semantics Other richTextFromDescriptor:)]
        pub unsafe fn richTextFromDescriptor(
            &self,
            descriptor: &NSAppleEventDescriptor,
        ) -> Option<Retained<NSAttributedString>>;

        #[method(writeToURL:ofType:error:)]
        pub unsafe fn writeToURL_ofType_error(
            &self,
            url: &NSURL,
            r#type: &NSString,
            error_info: Option<&mut Option<Retained<NSDictionary<NSString, AnyObject>>>>,
        ) -> bool;

        #[method(writeToURL:ofType:usingStorageOptions:error:)]
        pub unsafe fn writeToURL_ofType_usingStorageOptions_error(
            &self,
            url: &NSURL,
            r#type: &NSString,
            storage_options: OSAStorageOptions,
            error_info: Option<&mut Option<Retained<NSDictionary<NSString, AnyObject>>>>,
        ) -> bool;

        #[method_id(@__retain_semantics Other compiledDataForType:usingStorageOptions:error:)]
        pub unsafe fn compiledDataForType_usingStorageOptions_error(
            &self,
            r#type: &NSString,
            storage_options: OSAStorageOptions,
            error_info: Option<&mut Option<Retained<NSDictionary<NSString, AnyObject>>>>,
        ) -> Option<Retained<NSData>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl OSAScript {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
