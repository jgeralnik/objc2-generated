// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]

#[link(name = "Photos", kind = "framework")]
extern "C" {}

#[cfg(feature = "PHAdjustmentData")]
#[path = "PHAdjustmentData.rs"]
mod __PHAdjustmentData;
#[cfg(feature = "PHAsset")]
#[path = "PHAsset.rs"]
mod __PHAsset;
#[cfg(feature = "PHAssetChangeRequest")]
#[path = "PHAssetChangeRequest.rs"]
mod __PHAssetChangeRequest;
#[cfg(feature = "PHAssetCollectionChangeRequest")]
#[path = "PHAssetCollectionChangeRequest.rs"]
mod __PHAssetCollectionChangeRequest;
#[cfg(feature = "PHAssetCreationRequest")]
#[path = "PHAssetCreationRequest.rs"]
mod __PHAssetCreationRequest;
#[cfg(feature = "PHAssetResource")]
#[path = "PHAssetResource.rs"]
mod __PHAssetResource;
#[cfg(feature = "PHAssetResourceManager")]
#[path = "PHAssetResourceManager.rs"]
mod __PHAssetResourceManager;
#[cfg(feature = "PHChange")]
#[path = "PHChange.rs"]
mod __PHChange;
#[cfg(feature = "PHChangeRequest")]
#[path = "PHChangeRequest.rs"]
mod __PHChangeRequest;
#[cfg(feature = "PHCloudIdentifier")]
#[path = "PHCloudIdentifier.rs"]
mod __PHCloudIdentifier;
#[cfg(feature = "PHCollection")]
#[path = "PHCollection.rs"]
mod __PHCollection;
#[cfg(feature = "PHCollectionListChangeRequest")]
#[path = "PHCollectionListChangeRequest.rs"]
mod __PHCollectionListChangeRequest;
#[cfg(feature = "PHContentEditingInput")]
#[path = "PHContentEditingInput.rs"]
mod __PHContentEditingInput;
#[cfg(feature = "PHContentEditingOutput")]
#[path = "PHContentEditingOutput.rs"]
mod __PHContentEditingOutput;
#[cfg(feature = "PHError")]
#[path = "PHError.rs"]
mod __PHError;
#[cfg(feature = "PHFetchOptions")]
#[path = "PHFetchOptions.rs"]
mod __PHFetchOptions;
#[cfg(feature = "PHFetchResult")]
#[path = "PHFetchResult.rs"]
mod __PHFetchResult;
#[cfg(feature = "PHImageManager")]
#[path = "PHImageManager.rs"]
mod __PHImageManager;
#[cfg(feature = "PHLivePhoto")]
#[path = "PHLivePhoto.rs"]
mod __PHLivePhoto;
#[cfg(feature = "PHLivePhotoEditingContext")]
#[path = "PHLivePhotoEditingContext.rs"]
mod __PHLivePhotoEditingContext;
#[cfg(feature = "PHObject")]
#[path = "PHObject.rs"]
mod __PHObject;
#[cfg(feature = "PHPersistentChange")]
#[path = "PHPersistentChange.rs"]
mod __PHPersistentChange;
#[cfg(feature = "PHPersistentChangeFetchResult")]
#[path = "PHPersistentChangeFetchResult.rs"]
mod __PHPersistentChangeFetchResult;
#[cfg(feature = "PHPersistentChangeToken")]
#[path = "PHPersistentChangeToken.rs"]
mod __PHPersistentChangeToken;
#[cfg(feature = "PHPersistentObjectChangeDetails")]
#[path = "PHPersistentObjectChangeDetails.rs"]
mod __PHPersistentObjectChangeDetails;
#[cfg(feature = "PHPhotoLibrary")]
#[path = "PHPhotoLibrary.rs"]
mod __PHPhotoLibrary;
#[cfg(feature = "PHProject")]
#[path = "PHProject.rs"]
mod __PHProject;
#[cfg(feature = "PHProjectChangeRequest")]
#[path = "PHProjectChangeRequest.rs"]
mod __PHProjectChangeRequest;
#[cfg(feature = "PhotosTypes")]
#[path = "PhotosTypes.rs"]
mod __PhotosTypes;

#[cfg(feature = "PHAdjustmentData")]
pub use self::__PHAdjustmentData::PHAdjustmentData;
#[cfg(all(feature = "PHAsset", feature = "PHObject"))]
pub use self::__PHAsset::PHAsset;
#[cfg(all(feature = "PHAssetChangeRequest", feature = "PHChangeRequest"))]
pub use self::__PHAssetChangeRequest::PHAssetChangeRequest;
#[cfg(feature = "PHAssetChangeRequest")]
pub use self::__PHAssetChangeRequest::PHContentEditingInputCancelledKey;
#[cfg(feature = "PHAssetChangeRequest")]
pub use self::__PHAssetChangeRequest::PHContentEditingInputErrorKey;
#[cfg(feature = "PHAssetChangeRequest")]
pub use self::__PHAssetChangeRequest::PHContentEditingInputRequestID;
#[cfg(feature = "PHAssetChangeRequest")]
pub use self::__PHAssetChangeRequest::PHContentEditingInputRequestOptions;
#[cfg(feature = "PHAssetChangeRequest")]
pub use self::__PHAssetChangeRequest::PHContentEditingInputResultIsInCloudKey;
#[cfg(all(
    feature = "PHAssetCollectionChangeRequest",
    feature = "PHChangeRequest"
))]
pub use self::__PHAssetCollectionChangeRequest::PHAssetCollectionChangeRequest;
#[cfg(all(
    feature = "PHAssetChangeRequest",
    feature = "PHAssetCreationRequest",
    feature = "PHChangeRequest"
))]
pub use self::__PHAssetCreationRequest::PHAssetCreationRequest;
#[cfg(feature = "PHAssetCreationRequest")]
pub use self::__PHAssetCreationRequest::PHAssetResourceCreationOptions;
#[cfg(feature = "PHAssetResource")]
pub use self::__PHAssetResource::PHAssetResource;
#[cfg(feature = "PHAssetResourceManager")]
pub use self::__PHAssetResourceManager::PHAssetResourceDataRequestID;
#[cfg(feature = "PHAssetResourceManager")]
pub use self::__PHAssetResourceManager::PHAssetResourceManager;
#[cfg(all(feature = "PHAssetResourceManager", feature = "block2"))]
pub use self::__PHAssetResourceManager::PHAssetResourceProgressHandler;
#[cfg(feature = "PHAssetResourceManager")]
pub use self::__PHAssetResourceManager::PHAssetResourceRequestOptions;
#[cfg(feature = "PHAssetResourceManager")]
pub use self::__PHAssetResourceManager::PHInvalidAssetResourceDataRequestID;
#[cfg(feature = "PHChange")]
pub use self::__PHChange::PHChange;
#[cfg(feature = "PHChange")]
pub use self::__PHChange::PHFetchResultChangeDetails;
#[cfg(feature = "PHChange")]
pub use self::__PHChange::PHObjectChangeDetails;
#[cfg(feature = "PHChangeRequest")]
pub use self::__PHChangeRequest::PHChangeRequest;
#[cfg(feature = "PHCloudIdentifier")]
pub use self::__PHCloudIdentifier::PHCloudIdentifier;
#[cfg(feature = "PHCloudIdentifier")]
pub use self::__PHCloudIdentifier::PHCloudIdentifierMapping;
#[cfg(feature = "PHCloudIdentifier")]
pub use self::__PHCloudIdentifier::PHLocalIdentifierMapping;
#[cfg(feature = "PHCloudIdentifier")]
pub use self::__PHCloudIdentifier::PHLocalIdentifierNotFound;
#[cfg(all(feature = "PHCollection", feature = "PHObject"))]
pub use self::__PHCollection::PHAssetCollection;
#[cfg(all(feature = "PHCollection", feature = "PHObject"))]
pub use self::__PHCollection::PHCollection;
#[cfg(all(feature = "PHCollection", feature = "PHObject"))]
pub use self::__PHCollection::PHCollectionList;
#[cfg(all(feature = "PHChangeRequest", feature = "PHCollectionListChangeRequest"))]
pub use self::__PHCollectionListChangeRequest::PHCollectionListChangeRequest;
#[cfg(feature = "PHContentEditingInput")]
pub use self::__PHContentEditingInput::PHContentEditingInput;
#[cfg(feature = "PHContentEditingOutput")]
pub use self::__PHContentEditingOutput::PHContentEditingOutput;
#[cfg(feature = "PHError")]
pub use self::__PHError::PHLocalIdentifiersErrorKey;
#[cfg(feature = "PHError")]
pub use self::__PHError::PHPhotosError;
#[cfg(feature = "PHError")]
pub use self::__PHError::PHPhotosErrorDomain;
#[cfg(feature = "PHFetchOptions")]
pub use self::__PHFetchOptions::PHFetchOptions;
#[cfg(feature = "PHFetchResult")]
pub use self::__PHFetchResult::PHFetchResult;
#[cfg(all(feature = "PHImageManager", feature = "block2"))]
pub use self::__PHImageManager::PHAssetImageProgressHandler;
#[cfg(all(feature = "PHImageManager", feature = "block2"))]
pub use self::__PHImageManager::PHAssetVideoProgressHandler;
#[cfg(feature = "PHImageManager")]
pub use self::__PHImageManager::PHCachingImageManager;
#[cfg(feature = "PHImageManager")]
pub use self::__PHImageManager::PHImageCancelledKey;
#[cfg(feature = "PHImageManager")]
pub use self::__PHImageManager::PHImageErrorKey;
#[cfg(feature = "PHImageManager")]
pub use self::__PHImageManager::PHImageManager;
#[cfg(feature = "PHImageManager")]
pub use self::__PHImageManager::PHImageManagerMaximumSize;
#[cfg(feature = "PHImageManager")]
pub use self::__PHImageManager::PHImageRequestID;
#[cfg(feature = "PHImageManager")]
pub use self::__PHImageManager::PHImageRequestOptions;
#[cfg(feature = "PHImageManager")]
pub use self::__PHImageManager::PHImageRequestOptionsDeliveryMode;
#[cfg(feature = "PHImageManager")]
pub use self::__PHImageManager::PHImageRequestOptionsResizeMode;
#[cfg(feature = "PHImageManager")]
pub use self::__PHImageManager::PHImageRequestOptionsVersion;
#[cfg(feature = "PHImageManager")]
pub use self::__PHImageManager::PHImageResultIsDegradedKey;
#[cfg(feature = "PHImageManager")]
pub use self::__PHImageManager::PHImageResultIsInCloudKey;
#[cfg(feature = "PHImageManager")]
pub use self::__PHImageManager::PHImageResultRequestIDKey;
#[cfg(feature = "PHImageManager")]
pub use self::__PHImageManager::PHInvalidImageRequestID;
#[cfg(feature = "PHImageManager")]
pub use self::__PHImageManager::PHLivePhotoRequestOptions;
#[cfg(feature = "PHImageManager")]
pub use self::__PHImageManager::PHVideoRequestOptions;
#[cfg(feature = "PHImageManager")]
pub use self::__PHImageManager::PHVideoRequestOptionsDeliveryMode;
#[cfg(feature = "PHImageManager")]
pub use self::__PHImageManager::PHVideoRequestOptionsVersion;
#[cfg(feature = "PHLivePhoto")]
pub use self::__PHLivePhoto::PHLivePhoto;
#[cfg(feature = "PHLivePhoto")]
pub use self::__PHLivePhoto::PHLivePhotoInfoCancelledKey;
#[cfg(feature = "PHLivePhoto")]
pub use self::__PHLivePhoto::PHLivePhotoInfoErrorKey;
#[cfg(feature = "PHLivePhoto")]
pub use self::__PHLivePhoto::PHLivePhotoInfoIsDegradedKey;
#[cfg(feature = "PHLivePhoto")]
pub use self::__PHLivePhoto::PHLivePhotoRequestID;
#[cfg(feature = "PHLivePhoto")]
pub use self::__PHLivePhoto::PHLivePhotoRequestIDInvalid;
#[cfg(feature = "PHLivePhotoEditingContext")]
pub use self::__PHLivePhotoEditingContext::PHLivePhotoEditingContext;
#[cfg(feature = "PHLivePhotoEditingContext")]
pub use self::__PHLivePhotoEditingContext::PHLivePhotoEditingErrorCode;
#[cfg(feature = "PHLivePhotoEditingContext")]
pub use self::__PHLivePhotoEditingContext::PHLivePhotoEditingErrorDomain;
#[cfg(feature = "PHLivePhotoEditingContext")]
pub use self::__PHLivePhotoEditingContext::PHLivePhotoEditingOption;
#[cfg(feature = "PHLivePhotoEditingContext")]
pub use self::__PHLivePhotoEditingContext::PHLivePhotoFrame;
#[cfg(all(
    feature = "PHLivePhotoEditingContext",
    feature = "block2",
    feature = "objc2-core-image"
))]
pub use self::__PHLivePhotoEditingContext::PHLivePhotoFrameProcessingBlock;
#[cfg(feature = "PHLivePhotoEditingContext")]
pub use self::__PHLivePhotoEditingContext::PHLivePhotoFrameType;
#[cfg(feature = "PHLivePhotoEditingContext")]
pub use self::__PHLivePhotoEditingContext::PHLivePhotoShouldRenderAtPlaybackTime;
#[cfg(feature = "PHObject")]
pub use self::__PHObject::PHObject;
#[cfg(feature = "PHObject")]
pub use self::__PHObject::PHObjectPlaceholder;
#[cfg(feature = "PHPersistentChange")]
pub use self::__PHPersistentChange::PHPersistentChange;
#[cfg(feature = "PHPersistentChangeFetchResult")]
pub use self::__PHPersistentChangeFetchResult::PHPersistentChangeFetchResult;
#[cfg(feature = "PHPersistentChangeToken")]
pub use self::__PHPersistentChangeToken::PHPersistentChangeToken;
#[cfg(feature = "PHPersistentObjectChangeDetails")]
pub use self::__PHPersistentObjectChangeDetails::PHPersistentObjectChangeDetails;
#[cfg(feature = "PHPhotoLibrary")]
pub use self::__PHPhotoLibrary::PHAccessLevel;
#[cfg(feature = "PHPhotoLibrary")]
pub use self::__PHPhotoLibrary::PHAuthorizationStatus;
#[cfg(feature = "PHPhotoLibrary")]
pub use self::__PHPhotoLibrary::PHPhotoLibrary;
#[cfg(feature = "PHPhotoLibrary")]
pub use self::__PHPhotoLibrary::PHPhotoLibraryAvailabilityObserver;
#[cfg(feature = "PHPhotoLibrary")]
pub use self::__PHPhotoLibrary::PHPhotoLibraryChangeObserver;
#[cfg(all(feature = "PHCollection", feature = "PHObject", feature = "PHProject"))]
pub use self::__PHProject::PHProject;
#[cfg(all(feature = "PHChangeRequest", feature = "PHProjectChangeRequest"))]
pub use self::__PHProjectChangeRequest::PHProjectChangeRequest;
#[cfg(feature = "PhotosTypes")]
pub use self::__PhotosTypes::PHAssetBurstSelectionType;
#[cfg(feature = "PhotosTypes")]
pub use self::__PhotosTypes::PHAssetCollectionSubtype;
#[cfg(feature = "PhotosTypes")]
pub use self::__PhotosTypes::PHAssetCollectionType;
#[cfg(feature = "PhotosTypes")]
pub use self::__PhotosTypes::PHAssetEditOperation;
#[cfg(feature = "PhotosTypes")]
pub use self::__PhotosTypes::PHAssetMediaSubtype;
#[cfg(feature = "PhotosTypes")]
pub use self::__PhotosTypes::PHAssetMediaType;
#[cfg(feature = "PhotosTypes")]
pub use self::__PhotosTypes::PHAssetPlaybackStyle;
#[cfg(feature = "PhotosTypes")]
pub use self::__PhotosTypes::PHAssetResourceType;
#[cfg(feature = "PhotosTypes")]
pub use self::__PhotosTypes::PHAssetSourceType;
#[cfg(feature = "PhotosTypes")]
pub use self::__PhotosTypes::PHCollectionEditOperation;
#[cfg(feature = "PhotosTypes")]
pub use self::__PhotosTypes::PHCollectionListSubtype;
#[cfg(feature = "PhotosTypes")]
pub use self::__PhotosTypes::PHCollectionListType;
#[cfg(feature = "PhotosTypes")]
pub use self::__PhotosTypes::PHImageContentMode;
#[cfg(feature = "PhotosTypes")]
pub use self::__PhotosTypes::PHObjectType;
