//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFileWrapperReadingOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSFileWrapperReadingOptions: NSUInteger {
        const NSFileWrapperReadingImmediate = 1<<0;
        const NSFileWrapperReadingWithoutMapping = 1<<1;
    }
}

unsafe impl Encode for NSFileWrapperReadingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSFileWrapperReadingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFileWrapperWritingOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSFileWrapperWritingOptions: NSUInteger {
        const NSFileWrapperWritingAtomic = 1<<0;
        const NSFileWrapperWritingWithNameUpdating = 1<<1;
    }
}

unsafe impl Encode for NSFileWrapperWritingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSFileWrapperWritingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFileWrapper;
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSFileWrapper {}

unsafe impl NSObjectProtocol for NSFileWrapper {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSFileWrapper {}

extern_methods!(
    unsafe impl NSFileWrapper {
        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[method_id(@__retain_semantics Init initWithURL:options:error:_)]
        pub unsafe fn initWithURL_options_error(
            this: Allocated<Self>,
            url: &NSURL,
            options: NSFileWrapperReadingOptions,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Init initDirectoryWithFileWrappers:)]
        pub unsafe fn initDirectoryWithFileWrappers(
            this: Allocated<Self>,
            children_by_preferred_name: &NSDictionary<NSString, NSFileWrapper>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSData")]
        #[method_id(@__retain_semantics Init initRegularFileWithContents:)]
        pub unsafe fn initRegularFileWithContents(
            this: Allocated<Self>,
            contents: &NSData,
        ) -> Retained<Self>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Init initSymbolicLinkWithDestinationURL:)]
        pub unsafe fn initSymbolicLinkWithDestinationURL(
            this: Allocated<Self>,
            url: &NSURL,
        ) -> Retained<Self>;

        #[cfg(feature = "NSData")]
        #[method_id(@__retain_semantics Init initWithSerializedRepresentation:)]
        pub unsafe fn initWithSerializedRepresentation(
            this: Allocated<Self>,
            serialize_representation: &NSData,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            in_coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method(isDirectory)]
        pub unsafe fn isDirectory(&self) -> bool;

        #[method(isRegularFile)]
        pub unsafe fn isRegularFile(&self) -> bool;

        #[method(isSymbolicLink)]
        pub unsafe fn isSymbolicLink(&self) -> bool;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other preferredFilename)]
        pub unsafe fn preferredFilename(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method(setPreferredFilename:)]
        pub unsafe fn setPreferredFilename(&self, preferred_filename: Option<&NSString>);

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other filename)]
        pub unsafe fn filename(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method(setFilename:)]
        pub unsafe fn setFilename(&self, filename: Option<&NSString>);

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other fileAttributes)]
        pub unsafe fn fileAttributes(&self) -> Retained<NSDictionary<NSString, AnyObject>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method(setFileAttributes:)]
        pub unsafe fn setFileAttributes(&self, file_attributes: &NSDictionary<NSString, AnyObject>);

        #[cfg(feature = "NSURL")]
        #[method(matchesContentsOfURL:)]
        pub unsafe fn matchesContentsOfURL(&self, url: &NSURL) -> bool;

        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[method(readFromURL:options:error:_)]
        pub unsafe fn readFromURL_options_error(
            &self,
            url: &NSURL,
            options: NSFileWrapperReadingOptions,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[method(writeToURL:options:originalContentsURL:error:_)]
        pub unsafe fn writeToURL_options_originalContentsURL_error(
            &self,
            url: &NSURL,
            options: NSFileWrapperWritingOptions,
            original_contents_url: Option<&NSURL>,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "NSData")]
        #[method_id(@__retain_semantics Other serializedRepresentation)]
        pub unsafe fn serializedRepresentation(&self) -> Option<Retained<NSData>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other addFileWrapper:)]
        pub unsafe fn addFileWrapper(&self, child: &NSFileWrapper) -> Retained<NSString>;

        #[cfg(all(feature = "NSData", feature = "NSString"))]
        #[method_id(@__retain_semantics Other addRegularFileWithContents:preferredFilename:)]
        pub unsafe fn addRegularFileWithContents_preferredFilename(
            &self,
            data: &NSData,
            file_name: &NSString,
        ) -> Retained<NSString>;

        #[method(removeFileWrapper:)]
        pub unsafe fn removeFileWrapper(&self, child: &NSFileWrapper);

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other fileWrappers)]
        pub unsafe fn fileWrappers(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, NSFileWrapper>>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other keyForFileWrapper:)]
        pub unsafe fn keyForFileWrapper(&self, child: &NSFileWrapper)
            -> Option<Retained<NSString>>;

        #[cfg(feature = "NSData")]
        #[method_id(@__retain_semantics Other regularFileContents)]
        pub unsafe fn regularFileContents(&self) -> Option<Retained<NSData>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other symbolicLinkDestinationURL)]
        pub unsafe fn symbolicLinkDestinationURL(&self) -> Option<Retained<NSURL>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSFileWrapper {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSFileWrapper {
        #[cfg(feature = "NSString")]
        #[deprecated = "Use -initWithURL:options:error: instead."]
        #[method_id(@__retain_semantics Init initWithPath:)]
        pub unsafe fn initWithPath(
            this: Allocated<Self>,
            path: &NSString,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use -initSymbolicLinkWithDestinationURL: and -setPreferredFileName:, if necessary, instead."]
        #[method_id(@__retain_semantics Init initSymbolicLinkWithDestination:)]
        pub unsafe fn initSymbolicLinkWithDestination(
            this: Allocated<Self>,
            path: &NSString,
        ) -> Retained<Self>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use -matchesContentsOfURL: instead."]
        #[method(needsToBeUpdatedFromPath:)]
        pub unsafe fn needsToBeUpdatedFromPath(&self, path: &NSString) -> bool;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use -readFromURL:options:error: instead."]
        #[method(updateFromPath:)]
        pub unsafe fn updateFromPath(&self, path: &NSString) -> bool;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use -writeToURL:options:originalContentsURL:error: instead."]
        #[method(writeToFile:atomically:updateFilenames:)]
        pub unsafe fn writeToFile_atomically_updateFilenames(
            &self,
            path: &NSString,
            atomic_flag: bool,
            update_filenames_flag: bool,
        ) -> bool;

        #[cfg(feature = "NSString")]
        #[deprecated = "Instantiate a new NSFileWrapper with -initWithURL:options:error:, send it -setPreferredFileName: if necessary, then use -addFileWrapper: instead."]
        #[method_id(@__retain_semantics Other addFileWithPath:)]
        pub unsafe fn addFileWithPath(&self, path: &NSString) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Instantiate a new NSFileWrapper with -initWithSymbolicLinkDestinationURL:, send it -setPreferredFileName: if necessary, then use -addFileWrapper: instead."]
        #[method_id(@__retain_semantics Other addSymbolicLinkWithDestination:preferredFilename:)]
        pub unsafe fn addSymbolicLinkWithDestination_preferredFilename(
            &self,
            path: &NSString,
            filename: &NSString,
        ) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use -symbolicLinkDestinationURL instead."]
        #[method_id(@__retain_semantics Other symbolicLinkDestination)]
        pub unsafe fn symbolicLinkDestination(&self) -> Retained<NSString>;
    }
);
