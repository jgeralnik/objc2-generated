//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimagepickercontrollersourcetype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIImagePickerControllerSourceType(pub NSInteger);
impl UIImagePickerControllerSourceType {
    #[deprecated = "Will be removed in a future release, use PHPicker."]
    #[doc(alias = "UIImagePickerControllerSourceTypePhotoLibrary")]
    pub const PhotoLibrary: Self = Self(0);
    #[doc(alias = "UIImagePickerControllerSourceTypeCamera")]
    pub const Camera: Self = Self(1);
    #[deprecated = "Will be removed in a future release, use PHPicker."]
    #[doc(alias = "UIImagePickerControllerSourceTypeSavedPhotosAlbum")]
    pub const SavedPhotosAlbum: Self = Self(2);
}

unsafe impl Encode for UIImagePickerControllerSourceType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIImagePickerControllerSourceType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimagepickercontrollerqualitytype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIImagePickerControllerQualityType(pub NSInteger);
impl UIImagePickerControllerQualityType {
    #[doc(alias = "UIImagePickerControllerQualityTypeHigh")]
    pub const High: Self = Self(0);
    #[doc(alias = "UIImagePickerControllerQualityTypeMedium")]
    pub const Medium: Self = Self(1);
    #[doc(alias = "UIImagePickerControllerQualityTypeLow")]
    pub const Low: Self = Self(2);
    pub const UIImagePickerControllerQualityType640x480: Self = Self(3);
    #[doc(alias = "UIImagePickerControllerQualityTypeIFrame1280x720")]
    pub const IFrame1280x720: Self = Self(4);
    #[doc(alias = "UIImagePickerControllerQualityTypeIFrame960x540")]
    pub const IFrame960x540: Self = Self(5);
}

unsafe impl Encode for UIImagePickerControllerQualityType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIImagePickerControllerQualityType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimagepickercontrollercameracapturemode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIImagePickerControllerCameraCaptureMode(pub NSInteger);
impl UIImagePickerControllerCameraCaptureMode {
    #[doc(alias = "UIImagePickerControllerCameraCaptureModePhoto")]
    pub const Photo: Self = Self(0);
    #[doc(alias = "UIImagePickerControllerCameraCaptureModeVideo")]
    pub const Video: Self = Self(1);
}

unsafe impl Encode for UIImagePickerControllerCameraCaptureMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIImagePickerControllerCameraCaptureMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimagepickercontrollercameradevice?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIImagePickerControllerCameraDevice(pub NSInteger);
impl UIImagePickerControllerCameraDevice {
    #[doc(alias = "UIImagePickerControllerCameraDeviceRear")]
    pub const Rear: Self = Self(0);
    #[doc(alias = "UIImagePickerControllerCameraDeviceFront")]
    pub const Front: Self = Self(1);
}

unsafe impl Encode for UIImagePickerControllerCameraDevice {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIImagePickerControllerCameraDevice {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimagepickercontrollercameraflashmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIImagePickerControllerCameraFlashMode(pub NSInteger);
impl UIImagePickerControllerCameraFlashMode {
    #[doc(alias = "UIImagePickerControllerCameraFlashModeOff")]
    pub const Off: Self = Self(-1);
    #[doc(alias = "UIImagePickerControllerCameraFlashModeAuto")]
    pub const Auto: Self = Self(0);
    #[doc(alias = "UIImagePickerControllerCameraFlashModeOn")]
    pub const On: Self = Self(1);
}

unsafe impl Encode for UIImagePickerControllerCameraFlashMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIImagePickerControllerCameraFlashMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimagepickercontrollerimageurlexportpreset?language=objc)
// NS_ENUM
#[deprecated = "Will be removed in a future release, use PHPicker."]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIImagePickerControllerImageURLExportPreset(pub NSInteger);
impl UIImagePickerControllerImageURLExportPreset {
    #[deprecated = "Will be removed in a future release, use PHPicker."]
    #[doc(alias = "UIImagePickerControllerImageURLExportPresetCompatible")]
    pub const Compatible: Self = Self(0);
    #[deprecated = "Will be removed in a future release, use PHPicker."]
    #[doc(alias = "UIImagePickerControllerImageURLExportPresetCurrent")]
    pub const Current: Self = Self(1);
}

unsafe impl Encode for UIImagePickerControllerImageURLExportPreset {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIImagePickerControllerImageURLExportPreset {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimagepickercontrollerinfokey?language=objc)
// NS_TYPED_ENUM
pub type UIImagePickerControllerInfoKey = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimagepickercontrollermediatype?language=objc)
    pub static UIImagePickerControllerMediaType: &'static UIImagePickerControllerInfoKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimagepickercontrolleroriginalimage?language=objc)
    pub static UIImagePickerControllerOriginalImage: &'static UIImagePickerControllerInfoKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimagepickercontrollereditedimage?language=objc)
    pub static UIImagePickerControllerEditedImage: &'static UIImagePickerControllerInfoKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimagepickercontrollercroprect?language=objc)
    pub static UIImagePickerControllerCropRect: &'static UIImagePickerControllerInfoKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimagepickercontrollermediaurl?language=objc)
    pub static UIImagePickerControllerMediaURL: &'static UIImagePickerControllerInfoKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimagepickercontrollerreferenceurl?language=objc)
    pub static UIImagePickerControllerReferenceURL: &'static UIImagePickerControllerInfoKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimagepickercontrollermediametadata?language=objc)
    pub static UIImagePickerControllerMediaMetadata: &'static UIImagePickerControllerInfoKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimagepickercontrollerlivephoto?language=objc)
    pub static UIImagePickerControllerLivePhoto: &'static UIImagePickerControllerInfoKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimagepickercontrollerphasset?language=objc)
    pub static UIImagePickerControllerPHAsset: &'static UIImagePickerControllerInfoKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimagepickercontrollerimageurl?language=objc)
    pub static UIImagePickerControllerImageURL: &'static UIImagePickerControllerInfoKey;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimagepickercontroller?language=objc)
    #[unsafe(super(UINavigationController, UIViewController, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "UINavigationController",
        feature = "UIResponder",
        feature = "UIViewController"
    ))]
    pub struct UIImagePickerController;
);

#[cfg(all(
    feature = "UINavigationController",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl NSCoding for UIImagePickerController {}

#[cfg(all(
    feature = "UINavigationController",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl NSObjectProtocol for UIImagePickerController {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UINavigationController",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIAppearanceContainer for UIImagePickerController {}

#[cfg(all(
    feature = "UINavigationController",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIContentContainer for UIImagePickerController {}

#[cfg(all(
    feature = "UIFocus",
    feature = "UINavigationController",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIFocusEnvironment for UIImagePickerController {}

#[cfg(all(
    feature = "UINavigationController",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIResponderStandardEditActions for UIImagePickerController {}

#[cfg(all(
    feature = "UINavigationController",
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIViewController"
))]
unsafe impl UITraitEnvironment for UIImagePickerController {}

extern_methods!(
    #[cfg(all(
        feature = "UINavigationController",
        feature = "UIResponder",
        feature = "UIViewController"
    ))]
    unsafe impl UIImagePickerController {
        #[method(isSourceTypeAvailable:)]
        pub unsafe fn isSourceTypeAvailable(
            source_type: UIImagePickerControllerSourceType,
            mtm: MainThreadMarker,
        ) -> bool;

        #[method_id(@__retain_semantics Other availableMediaTypesForSourceType:)]
        pub unsafe fn availableMediaTypesForSourceType(
            source_type: UIImagePickerControllerSourceType,
            mtm: MainThreadMarker,
        ) -> Option<Retained<NSArray<NSString>>>;

        #[method(isCameraDeviceAvailable:)]
        pub unsafe fn isCameraDeviceAvailable(
            camera_device: UIImagePickerControllerCameraDevice,
            mtm: MainThreadMarker,
        ) -> bool;

        #[method(isFlashAvailableForCameraDevice:)]
        pub unsafe fn isFlashAvailableForCameraDevice(
            camera_device: UIImagePickerControllerCameraDevice,
            mtm: MainThreadMarker,
        ) -> bool;

        #[method_id(@__retain_semantics Other availableCaptureModesForCameraDevice:)]
        pub unsafe fn availableCaptureModesForCameraDevice(
            camera_device: UIImagePickerControllerCameraDevice,
            mtm: MainThreadMarker,
        ) -> Option<Retained<NSArray<NSNumber>>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<
            Retained<
                AnyObject, /* UINavigationControllerDelegate+ UIImagePickerControllerDelegate */
            >,
        >;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<
                &AnyObject, /* UINavigationControllerDelegate+ UIImagePickerControllerDelegate */
            >,
        );

        #[method(sourceType)]
        pub unsafe fn sourceType(&self) -> UIImagePickerControllerSourceType;

        #[method(setSourceType:)]
        pub unsafe fn setSourceType(&self, source_type: UIImagePickerControllerSourceType);

        #[method_id(@__retain_semantics Other mediaTypes)]
        pub unsafe fn mediaTypes(&self) -> Retained<NSArray<NSString>>;

        #[method(setMediaTypes:)]
        pub unsafe fn setMediaTypes(&self, media_types: &NSArray<NSString>);

        #[method(allowsEditing)]
        pub unsafe fn allowsEditing(&self) -> bool;

        #[method(setAllowsEditing:)]
        pub unsafe fn setAllowsEditing(&self, allows_editing: bool);

        #[deprecated]
        #[method(allowsImageEditing)]
        pub unsafe fn allowsImageEditing(&self) -> bool;

        #[deprecated]
        #[method(setAllowsImageEditing:)]
        pub unsafe fn setAllowsImageEditing(&self, allows_image_editing: bool);

        #[deprecated = "Will be removed in a future release, use PHPicker."]
        #[method(imageExportPreset)]
        pub unsafe fn imageExportPreset(&self) -> UIImagePickerControllerImageURLExportPreset;

        #[deprecated = "Will be removed in a future release, use PHPicker."]
        #[method(setImageExportPreset:)]
        pub unsafe fn setImageExportPreset(
            &self,
            image_export_preset: UIImagePickerControllerImageURLExportPreset,
        );

        #[method(videoMaximumDuration)]
        pub unsafe fn videoMaximumDuration(&self) -> NSTimeInterval;

        #[method(setVideoMaximumDuration:)]
        pub unsafe fn setVideoMaximumDuration(&self, video_maximum_duration: NSTimeInterval);

        #[method(videoQuality)]
        pub unsafe fn videoQuality(&self) -> UIImagePickerControllerQualityType;

        #[method(setVideoQuality:)]
        pub unsafe fn setVideoQuality(&self, video_quality: UIImagePickerControllerQualityType);

        #[deprecated = "Will be removed in a future release, use PHPicker."]
        #[method_id(@__retain_semantics Other videoExportPreset)]
        pub unsafe fn videoExportPreset(&self) -> Retained<NSString>;

        #[deprecated = "Will be removed in a future release, use PHPicker."]
        #[method(setVideoExportPreset:)]
        pub unsafe fn setVideoExportPreset(&self, video_export_preset: &NSString);

        #[method(showsCameraControls)]
        pub unsafe fn showsCameraControls(&self) -> bool;

        #[method(setShowsCameraControls:)]
        pub unsafe fn setShowsCameraControls(&self, shows_camera_controls: bool);

        #[cfg(feature = "UIView")]
        #[method_id(@__retain_semantics Other cameraOverlayView)]
        pub unsafe fn cameraOverlayView(&self) -> Option<Retained<UIView>>;

        #[cfg(feature = "UIView")]
        #[method(setCameraOverlayView:)]
        pub unsafe fn setCameraOverlayView(&self, camera_overlay_view: Option<&UIView>);

        #[method(cameraViewTransform)]
        pub unsafe fn cameraViewTransform(&self) -> CGAffineTransform;

        #[method(setCameraViewTransform:)]
        pub unsafe fn setCameraViewTransform(&self, camera_view_transform: CGAffineTransform);

        #[method(takePicture)]
        pub unsafe fn takePicture(&self);

        #[method(startVideoCapture)]
        pub unsafe fn startVideoCapture(&self) -> bool;

        #[method(stopVideoCapture)]
        pub unsafe fn stopVideoCapture(&self);

        #[method(cameraCaptureMode)]
        pub unsafe fn cameraCaptureMode(&self) -> UIImagePickerControllerCameraCaptureMode;

        #[method(setCameraCaptureMode:)]
        pub unsafe fn setCameraCaptureMode(
            &self,
            camera_capture_mode: UIImagePickerControllerCameraCaptureMode,
        );

        #[method(cameraDevice)]
        pub unsafe fn cameraDevice(&self) -> UIImagePickerControllerCameraDevice;

        #[method(setCameraDevice:)]
        pub unsafe fn setCameraDevice(&self, camera_device: UIImagePickerControllerCameraDevice);

        #[method(cameraFlashMode)]
        pub unsafe fn cameraFlashMode(&self) -> UIImagePickerControllerCameraFlashMode;

        #[method(setCameraFlashMode:)]
        pub unsafe fn setCameraFlashMode(
            &self,
            camera_flash_mode: UIImagePickerControllerCameraFlashMode,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `UINavigationController`
    #[cfg(all(
        feature = "UINavigationController",
        feature = "UIResponder",
        feature = "UIViewController"
    ))]
    unsafe impl UIImagePickerController {
        #[method_id(@__retain_semantics Init initWithNavigationBarClass:toolbarClass:)]
        pub unsafe fn initWithNavigationBarClass_toolbarClass(
            this: Allocated<Self>,
            navigation_bar_class: Option<&AnyClass>,
            toolbar_class: Option<&AnyClass>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithRootViewController:)]
        pub unsafe fn initWithRootViewController(
            this: Allocated<Self>,
            root_view_controller: &UIViewController,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSString>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "UINavigationController",
        feature = "UIResponder",
        feature = "UIViewController"
    ))]
    unsafe impl UIImagePickerController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiimagepickercontrollerdelegate?language=objc)
    pub unsafe trait UIImagePickerControllerDelegate:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(all(
            feature = "UIImage",
            feature = "UINavigationController",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[deprecated]
        #[optional]
        #[method(imagePickerController:didFinishPickingImage:editingInfo:)]
        unsafe fn imagePickerController_didFinishPickingImage_editingInfo(
            &self,
            picker: &UIImagePickerController,
            image: &UIImage,
            editing_info: Option<&NSDictionary<UIImagePickerControllerInfoKey, AnyObject>>,
        );

        #[cfg(all(
            feature = "UINavigationController",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(imagePickerController:didFinishPickingMediaWithInfo:)]
        unsafe fn imagePickerController_didFinishPickingMediaWithInfo(
            &self,
            picker: &UIImagePickerController,
            info: &NSDictionary<UIImagePickerControllerInfoKey, AnyObject>,
        );

        #[cfg(all(
            feature = "UINavigationController",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(imagePickerControllerDidCancel:)]
        unsafe fn imagePickerControllerDidCancel(&self, picker: &UIImagePickerController);
    }

    unsafe impl ProtocolType for dyn UIImagePickerControllerDelegate {}
);

extern "C-unwind" {
    #[cfg(feature = "UIImage")]
    pub fn UIImageWriteToSavedPhotosAlbum(
        image: &UIImage,
        completion_target: Option<&AnyObject>,
        completion_selector: Option<Sel>,
        context_info: *mut c_void,
    );
}

extern "C-unwind" {
    pub fn UIVideoAtPathIsCompatibleWithSavedPhotosAlbum(video_path: &NSString) -> Bool;
}

extern "C-unwind" {
    pub fn UISaveVideoAtPathToSavedPhotosAlbum(
        video_path: &NSString,
        completion_target: Option<&AnyObject>,
        completion_selector: Option<Sel>,
        context_info: *mut c_void,
    );
}
