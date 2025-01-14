//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-cloud-kit")]
use objc2_cloud_kit::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicloudsharingpermissionoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UICloudSharingPermissionOptions(pub NSUInteger);
bitflags::bitflags! {
    impl UICloudSharingPermissionOptions: NSUInteger {
        const UICloudSharingPermissionStandard = 0;
        const UICloudSharingPermissionAllowPublic = 1<<0;
        const UICloudSharingPermissionAllowPrivate = 1<<1;
        const UICloudSharingPermissionAllowReadOnly = 1<<2;
        const UICloudSharingPermissionAllowReadWrite = 1<<3;
    }
}

unsafe impl Encode for UICloudSharingPermissionOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UICloudSharingPermissionOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicloudsharingcontrollerdelegate?language=objc)
    pub unsafe trait UICloudSharingControllerDelegate:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[method(cloudSharingController:failedToSaveShareWithError:)]
        unsafe fn cloudSharingController_failedToSaveShareWithError(
            &self,
            csc: &UICloudSharingController,
            error: &NSError,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[method_id(@__retain_semantics Other itemTitleForCloudSharingController:)]
        unsafe fn itemTitleForCloudSharingController(
            &self,
            csc: &UICloudSharingController,
        ) -> Option<Retained<NSString>>;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method_id(@__retain_semantics Other itemThumbnailDataForCloudSharingController:)]
        unsafe fn itemThumbnailDataForCloudSharingController(
            &self,
            csc: &UICloudSharingController,
        ) -> Option<Retained<NSData>>;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method_id(@__retain_semantics Other itemTypeForCloudSharingController:)]
        unsafe fn itemTypeForCloudSharingController(
            &self,
            csc: &UICloudSharingController,
        ) -> Option<Retained<NSString>>;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(cloudSharingControllerDidSaveShare:)]
        unsafe fn cloudSharingControllerDidSaveShare(&self, csc: &UICloudSharingController);

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(cloudSharingControllerDidStopSharing:)]
        unsafe fn cloudSharingControllerDidStopSharing(&self, csc: &UICloudSharingController);
    }

    unsafe impl ProtocolType for dyn UICloudSharingControllerDelegate {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicloudsharingcontroller?language=objc)
    #[unsafe(super(UIViewController, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    pub struct UICloudSharingController;
);

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSCoding for UICloudSharingController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSObjectProtocol for UICloudSharingController {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIAppearanceContainer for UICloudSharingController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIContentContainer for UICloudSharingController {}

#[cfg(all(
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIFocusEnvironment for UICloudSharingController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIResponderStandardEditActions for UICloudSharingController {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIViewController"
))]
unsafe impl UITraitEnvironment for UICloudSharingController {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UICloudSharingController {
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSString>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "block2", feature = "objc2-cloud-kit"))]
        #[deprecated = "Use -[UIActivityViewController initWithActivityItemsConfiguration:] and pass it a UIActivityItemsConfigurationReading-conforming object with an NSItemProvider and registered preparation handler"]
        #[method_id(@__retain_semantics Init initWithPreparationHandler:)]
        pub unsafe fn initWithPreparationHandler(
            this: Allocated<Self>,
            preparation_handler: &block2::Block<
                dyn Fn(
                    NonNull<UICloudSharingController>,
                    NonNull<block2::Block<dyn Fn(*mut CKShare, *mut CKContainer, *mut NSError)>>,
                ),
            >,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-cloud-kit")]
        #[method_id(@__retain_semantics Init initWithShare:container:)]
        pub unsafe fn initWithShare_container(
            this: Allocated<Self>,
            share: &CKShare,
            container: &CKContainer,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UICloudSharingControllerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UICloudSharingControllerDelegate>>,
        );

        #[cfg(feature = "objc2-cloud-kit")]
        #[method_id(@__retain_semantics Other share)]
        pub unsafe fn share(&self) -> Option<Retained<CKShare>>;

        #[method(availablePermissions)]
        pub unsafe fn availablePermissions(&self) -> UICloudSharingPermissionOptions;

        #[method(setAvailablePermissions:)]
        pub unsafe fn setAvailablePermissions(
            &self,
            available_permissions: UICloudSharingPermissionOptions,
        );

        #[cfg(feature = "UIActivityItemProvider")]
        #[method_id(@__retain_semantics Other activityItemSource)]
        pub unsafe fn activityItemSource(
            &self,
        ) -> Retained<ProtocolObject<dyn UIActivityItemSource>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UICloudSharingController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
