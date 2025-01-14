//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-uniform-type-identifiers")]
use objc2_uniform_type_identifiers::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileprovideritemidentifier?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type NSFileProviderItemIdentifier = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileproviderrootcontaineritemidentifier?language=objc)
    pub static NSFileProviderRootContainerItemIdentifier: &'static NSFileProviderItemIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileproviderworkingsetcontaineritemidentifier?language=objc)
    pub static NSFileProviderWorkingSetContainerItemIdentifier:
        &'static NSFileProviderItemIdentifier;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileprovidertrashcontaineritemidentifier?language=objc)
    pub static NSFileProviderTrashContainerItemIdentifier: &'static NSFileProviderItemIdentifier;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileprovideritemversion?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFileProviderItemVersion;
);

unsafe impl NSObjectProtocol for NSFileProviderItemVersion {}

extern_methods!(
    unsafe impl NSFileProviderItemVersion {
        #[method_id(@__retain_semantics Other beforeFirstSyncComponent)]
        pub unsafe fn beforeFirstSyncComponent() -> Retained<NSData>;

        #[method_id(@__retain_semantics Init initWithContentVersion:metadataVersion:)]
        pub unsafe fn initWithContentVersion_metadataVersion(
            this: Allocated<Self>,
            content_version: &NSData,
            metadata_version: &NSData,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other contentVersion)]
        pub unsafe fn contentVersion(&self) -> Retained<NSData>;

        #[method_id(@__retain_semantics Other metadataVersion)]
        pub unsafe fn metadataVersion(&self) -> Retained<NSData>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSFileProviderItemVersion {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileproviderfavoriterankunranked?language=objc)
    pub static NSFileProviderFavoriteRankUnranked: c_ulonglong;
}

/// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileprovideritemcapabilities?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFileProviderItemCapabilities(pub NSUInteger);
bitflags::bitflags! {
    impl NSFileProviderItemCapabilities: NSUInteger {
        #[doc(alias = "NSFileProviderItemCapabilitiesAllowsReading")]
        const AllowsReading = 1<<0;
        #[doc(alias = "NSFileProviderItemCapabilitiesAllowsWriting")]
        const AllowsWriting = 1<<1;
        #[doc(alias = "NSFileProviderItemCapabilitiesAllowsReparenting")]
        const AllowsReparenting = 1<<2;
        #[doc(alias = "NSFileProviderItemCapabilitiesAllowsRenaming")]
        const AllowsRenaming = 1<<3;
        #[doc(alias = "NSFileProviderItemCapabilitiesAllowsTrashing")]
        const AllowsTrashing = 1<<4;
        #[doc(alias = "NSFileProviderItemCapabilitiesAllowsDeleting")]
        const AllowsDeleting = 1<<5;
#[deprecated = "use NSFileProviderContentPolicy instead"]
        #[doc(alias = "NSFileProviderItemCapabilitiesAllowsEvicting")]
        const AllowsEvicting = 1<<6;
        #[doc(alias = "NSFileProviderItemCapabilitiesAllowsExcludingFromSync")]
        const AllowsExcludingFromSync = 1<<7;
        #[doc(alias = "NSFileProviderItemCapabilitiesAllowsAddingSubItems")]
        const AllowsAddingSubItems = NSFileProviderItemCapabilities::AllowsWriting.0;
        #[doc(alias = "NSFileProviderItemCapabilitiesAllowsContentEnumerating")]
        const AllowsContentEnumerating = NSFileProviderItemCapabilities::AllowsReading.0;
#[deprecated = "This capability is no longer supported, and does not contain all capabilities. Please migrate to directly specifying each of the individual capabilities that should be allowed for the item."]
        #[doc(alias = "NSFileProviderItemCapabilitiesAllowsAll")]
        const AllowsAll = NSFileProviderItemCapabilities::AllowsReading.0|NSFileProviderItemCapabilities::AllowsWriting.0|NSFileProviderItemCapabilities::AllowsReparenting.0|NSFileProviderItemCapabilities::AllowsRenaming.0|NSFileProviderItemCapabilities::AllowsTrashing.0|NSFileProviderItemCapabilities::AllowsDeleting.0;
    }
}

unsafe impl Encode for NSFileProviderItemCapabilities {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSFileProviderItemCapabilities {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileprovideritemfields?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFileProviderItemFields(pub NSUInteger);
bitflags::bitflags! {
    impl NSFileProviderItemFields: NSUInteger {
        const NSFileProviderItemContents = 1<<0;
        const NSFileProviderItemFilename = 1<<1;
        const NSFileProviderItemParentItemIdentifier = 1<<2;
        const NSFileProviderItemLastUsedDate = 1<<3;
        const NSFileProviderItemTagData = 1<<4;
        const NSFileProviderItemFavoriteRank = 1<<5;
        const NSFileProviderItemCreationDate = 1<<6;
        const NSFileProviderItemContentModificationDate = 1<<7;
        const NSFileProviderItemFileSystemFlags = 1<<8;
        const NSFileProviderItemExtendedAttributes = 1<<9;
        const NSFileProviderItemTypeAndCreator = 1<<10;
    }
}

unsafe impl Encode for NSFileProviderItemFields {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSFileProviderItemFields {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileproviderfilesystemflags?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFileProviderFileSystemFlags(pub NSUInteger);
bitflags::bitflags! {
    impl NSFileProviderFileSystemFlags: NSUInteger {
        const NSFileProviderFileSystemUserExecutable = 1<<0;
        const NSFileProviderFileSystemUserReadable = 1<<1;
        const NSFileProviderFileSystemUserWritable = 1<<2;
        const NSFileProviderFileSystemHidden = 1<<3;
        const NSFileProviderFileSystemPathExtensionHidden = 1<<4;
    }
}

unsafe impl Encode for NSFileProviderFileSystemFlags {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSFileProviderFileSystemFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileprovidertypeandcreator?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NSFileProviderTypeAndCreator {
    pub r#type: OSType,
    pub creator: OSType,
}

unsafe impl Encode for NSFileProviderTypeAndCreator {
    const ENCODING: Encoding = Encoding::Struct(
        "NSFileProviderTypeAndCreator",
        &[<OSType>::ENCODING, <OSType>::ENCODING],
    );
}

unsafe impl RefEncode for NSFileProviderTypeAndCreator {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileprovidercontentpolicy?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFileProviderContentPolicy(pub NSInteger);
impl NSFileProviderContentPolicy {
    #[doc(alias = "NSFileProviderContentPolicyInherited")]
    pub const Inherited: Self = Self(0);
    #[doc(alias = "NSFileProviderContentPolicyDownloadLazily")]
    pub const DownloadLazily: Self = Self(1);
    #[doc(alias = "NSFileProviderContentPolicyDownloadLazilyAndEvictOnRemoteUpdate")]
    pub const DownloadLazilyAndEvictOnRemoteUpdate: Self = Self(2);
    #[doc(alias = "NSFileProviderContentPolicyDownloadEagerlyAndKeepDownloaded")]
    pub const DownloadEagerlyAndKeepDownloaded: Self = Self(3);
}

unsafe impl Encode for NSFileProviderContentPolicy {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSFileProviderContentPolicy {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileprovideritemprotocol?language=objc)
    pub unsafe trait NSFileProviderItemProtocol: NSObjectProtocol {
        #[method_id(@__retain_semantics Other itemIdentifier)]
        unsafe fn itemIdentifier(&self) -> Retained<NSFileProviderItemIdentifier>;

        #[method_id(@__retain_semantics Other parentItemIdentifier)]
        unsafe fn parentItemIdentifier(&self) -> Retained<NSFileProviderItemIdentifier>;

        #[method_id(@__retain_semantics Other filename)]
        unsafe fn filename(&self) -> Retained<NSString>;

        #[cfg(feature = "objc2-uniform-type-identifiers")]
        #[optional]
        #[method_id(@__retain_semantics Other contentType)]
        unsafe fn contentType(&self) -> Retained<UTType>;

        #[deprecated]
        #[optional]
        #[method_id(@__retain_semantics Other typeIdentifier)]
        unsafe fn typeIdentifier(&self) -> Retained<NSString>;

        #[optional]
        #[method(typeAndCreator)]
        unsafe fn typeAndCreator(&self) -> NSFileProviderTypeAndCreator;

        #[optional]
        #[method(capabilities)]
        unsafe fn capabilities(&self) -> NSFileProviderItemCapabilities;

        #[optional]
        #[method(fileSystemFlags)]
        unsafe fn fileSystemFlags(&self) -> NSFileProviderFileSystemFlags;

        #[optional]
        #[method_id(@__retain_semantics Other documentSize)]
        unsafe fn documentSize(&self) -> Option<Retained<NSNumber>>;

        #[optional]
        #[method_id(@__retain_semantics Other childItemCount)]
        unsafe fn childItemCount(&self) -> Option<Retained<NSNumber>>;

        #[optional]
        #[method_id(@__retain_semantics Other creationDate)]
        unsafe fn creationDate(&self) -> Option<Retained<NSDate>>;

        #[optional]
        #[method_id(@__retain_semantics Other contentModificationDate)]
        unsafe fn contentModificationDate(&self) -> Option<Retained<NSDate>>;

        #[optional]
        #[method_id(@__retain_semantics Other extendedAttributes)]
        unsafe fn extendedAttributes(&self) -> Retained<NSDictionary<NSString, NSData>>;

        #[optional]
        #[method_id(@__retain_semantics Other lastUsedDate)]
        unsafe fn lastUsedDate(&self) -> Option<Retained<NSDate>>;

        #[optional]
        #[method_id(@__retain_semantics Other tagData)]
        unsafe fn tagData(&self) -> Option<Retained<NSData>>;

        #[optional]
        #[method_id(@__retain_semantics Other favoriteRank)]
        unsafe fn favoriteRank(&self) -> Option<Retained<NSNumber>>;

        #[optional]
        #[method(isTrashed)]
        unsafe fn isTrashed(&self) -> bool;

        #[optional]
        #[method(isUploaded)]
        unsafe fn isUploaded(&self) -> bool;

        #[optional]
        #[method(isUploading)]
        unsafe fn isUploading(&self) -> bool;

        #[optional]
        #[method_id(@__retain_semantics Other uploadingError)]
        unsafe fn uploadingError(&self) -> Option<Retained<NSError>>;

        #[optional]
        #[method(isDownloaded)]
        unsafe fn isDownloaded(&self) -> bool;

        #[optional]
        #[method(isDownloading)]
        unsafe fn isDownloading(&self) -> bool;

        #[optional]
        #[method_id(@__retain_semantics Other downloadingError)]
        unsafe fn downloadingError(&self) -> Option<Retained<NSError>>;

        #[optional]
        #[method(isMostRecentVersionDownloaded)]
        unsafe fn isMostRecentVersionDownloaded(&self) -> bool;

        #[optional]
        #[method(isShared)]
        unsafe fn isShared(&self) -> bool;

        #[optional]
        #[method(isSharedByCurrentUser)]
        unsafe fn isSharedByCurrentUser(&self) -> bool;

        #[optional]
        #[method_id(@__retain_semantics Other ownerNameComponents)]
        unsafe fn ownerNameComponents(&self) -> Option<Retained<NSPersonNameComponents>>;

        #[optional]
        #[method_id(@__retain_semantics Other mostRecentEditorNameComponents)]
        unsafe fn mostRecentEditorNameComponents(&self)
            -> Option<Retained<NSPersonNameComponents>>;

        #[optional]
        #[method_id(@__retain_semantics Other versionIdentifier)]
        unsafe fn versionIdentifier(&self) -> Option<Retained<NSData>>;

        #[optional]
        #[method_id(@__retain_semantics Other itemVersion)]
        unsafe fn itemVersion(&self) -> Retained<NSFileProviderItemVersion>;

        #[optional]
        #[method_id(@__retain_semantics Other symlinkTargetPath)]
        unsafe fn symlinkTargetPath(&self) -> Option<Retained<NSString>>;

        #[optional]
        #[method_id(@__retain_semantics Other userInfo)]
        unsafe fn userInfo(&self) -> Option<Retained<NSDictionary>>;

        #[optional]
        #[method(contentPolicy)]
        unsafe fn contentPolicy(&self) -> NSFileProviderContentPolicy;
    }

    unsafe impl ProtocolType for dyn NSFileProviderItemProtocol {
        const NAME: &'static str = "NSFileProviderItem";
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileprovideritem?language=objc)
pub type NSFileProviderItem = ProtocolObject<dyn NSFileProviderItemProtocol>;
