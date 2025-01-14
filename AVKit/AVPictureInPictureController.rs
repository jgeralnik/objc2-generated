//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avkit/avpictureinpicturecontroller?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVPictureInPictureController;
);

unsafe impl NSObjectProtocol for AVPictureInPictureController {}

extern_methods!(
    unsafe impl AVPictureInPictureController {
        #[method(isPictureInPictureSupported)]
        pub unsafe fn isPictureInPictureSupported() -> bool;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Other pictureInPictureButtonStartImage)]
        pub unsafe fn pictureInPictureButtonStartImage() -> Retained<NSImage>;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Other pictureInPictureButtonStopImage)]
        pub unsafe fn pictureInPictureButtonStopImage() -> Retained<NSImage>;

        #[method_id(@__retain_semantics Init initWithContentSource:)]
        pub unsafe fn initWithContentSource(
            this: Allocated<Self>,
            content_source: &AVPictureInPictureControllerContentSource,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other contentSource)]
        pub unsafe fn contentSource(
            &self,
        ) -> Option<Retained<AVPictureInPictureControllerContentSource>>;

        #[method(setContentSource:)]
        pub unsafe fn setContentSource(
            &self,
            content_source: Option<&AVPictureInPictureControllerContentSource>,
        );

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn AVPictureInPictureControllerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn AVPictureInPictureControllerDelegate>>,
        );

        #[method(startPictureInPicture)]
        pub unsafe fn startPictureInPicture(&self);

        #[method(stopPictureInPicture)]
        pub unsafe fn stopPictureInPicture(&self);

        #[method(isPictureInPicturePossible)]
        pub unsafe fn isPictureInPicturePossible(&self) -> bool;

        #[method(isPictureInPictureActive)]
        pub unsafe fn isPictureInPictureActive(&self) -> bool;

        #[method(isPictureInPictureSuspended)]
        pub unsafe fn isPictureInPictureSuspended(&self) -> bool;

        #[method(canStopPictureInPicture)]
        pub unsafe fn canStopPictureInPicture(&self) -> bool;

        #[method(requiresLinearPlayback)]
        pub unsafe fn requiresLinearPlayback(&self) -> bool;

        #[method(setRequiresLinearPlayback:)]
        pub unsafe fn setRequiresLinearPlayback(&self, requires_linear_playback: bool);

        #[method(canStartPictureInPictureAutomaticallyFromInline)]
        pub unsafe fn canStartPictureInPictureAutomaticallyFromInline(&self) -> bool;

        #[method(setCanStartPictureInPictureAutomaticallyFromInline:)]
        pub unsafe fn setCanStartPictureInPictureAutomaticallyFromInline(
            &self,
            can_start_picture_in_picture_automatically_from_inline: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVPictureInPictureController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avkit/avpictureinpicturecontrollercontentsource?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVPictureInPictureControllerContentSource;
);

unsafe impl NSObjectProtocol for AVPictureInPictureControllerContentSource {}

extern_methods!(
    unsafe impl AVPictureInPictureControllerContentSource {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avkit/avpictureinpicturecontrollerdelegate?language=objc)
    pub unsafe trait AVPictureInPictureControllerDelegate: NSObjectProtocol {
        #[optional]
        #[method(pictureInPictureControllerWillStartPictureInPicture:)]
        unsafe fn pictureInPictureControllerWillStartPictureInPicture(
            &self,
            picture_in_picture_controller: &AVPictureInPictureController,
        );

        #[optional]
        #[method(pictureInPictureControllerDidStartPictureInPicture:)]
        unsafe fn pictureInPictureControllerDidStartPictureInPicture(
            &self,
            picture_in_picture_controller: &AVPictureInPictureController,
        );

        #[optional]
        #[method(pictureInPictureController:failedToStartPictureInPictureWithError:)]
        unsafe fn pictureInPictureController_failedToStartPictureInPictureWithError(
            &self,
            picture_in_picture_controller: &AVPictureInPictureController,
            error: &NSError,
        );

        #[optional]
        #[method(pictureInPictureControllerWillStopPictureInPicture:)]
        unsafe fn pictureInPictureControllerWillStopPictureInPicture(
            &self,
            picture_in_picture_controller: &AVPictureInPictureController,
        );

        #[optional]
        #[method(pictureInPictureControllerDidStopPictureInPicture:)]
        unsafe fn pictureInPictureControllerDidStopPictureInPicture(
            &self,
            picture_in_picture_controller: &AVPictureInPictureController,
        );

        #[cfg(feature = "block2")]
        #[optional]
        #[method(pictureInPictureController:restoreUserInterfaceForPictureInPictureStopWithCompletionHandler:)]
        unsafe fn pictureInPictureController_restoreUserInterfaceForPictureInPictureStopWithCompletionHandler(
            &self,
            picture_in_picture_controller: &AVPictureInPictureController,
            completion_handler: &block2::Block<dyn Fn(Bool)>,
        );
    }

    unsafe impl ProtocolType for dyn AVPictureInPictureControllerDelegate {}
);
