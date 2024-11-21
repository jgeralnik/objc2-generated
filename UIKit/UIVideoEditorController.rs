//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(UINavigationController, UIViewController, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "UINavigationController",
        feature = "UIResponder",
        feature = "UIViewController"
    ))]
    pub struct UIVideoEditorController;
);

#[cfg(all(
    feature = "UINavigationController",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl NSCoding for UIVideoEditorController {}

#[cfg(all(
    feature = "UINavigationController",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl NSObjectProtocol for UIVideoEditorController {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UINavigationController",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIAppearanceContainer for UIVideoEditorController {}

#[cfg(all(
    feature = "UINavigationController",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIContentContainer for UIVideoEditorController {}

#[cfg(all(
    feature = "UIFocus",
    feature = "UINavigationController",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIFocusEnvironment for UIVideoEditorController {}

#[cfg(all(
    feature = "UINavigationController",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIResponderStandardEditActions for UIVideoEditorController {}

#[cfg(all(
    feature = "UINavigationController",
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIViewController"
))]
unsafe impl UITraitEnvironment for UIVideoEditorController {}

extern_methods!(
    #[cfg(all(
        feature = "UINavigationController",
        feature = "UIResponder",
        feature = "UIViewController"
    ))]
    unsafe impl UIVideoEditorController {
        #[method(canEditVideoAtPath:)]
        pub unsafe fn canEditVideoAtPath(video_path: &NSString, mtm: MainThreadMarker) -> bool;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<TodoProtocols>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&TodoProtocols>);

        #[method_id(@__retain_semantics Other videoPath)]
        pub unsafe fn videoPath(&self) -> Retained<NSString>;

        #[method(setVideoPath:)]
        pub unsafe fn setVideoPath(&self, video_path: &NSString);

        #[method(videoMaximumDuration)]
        pub unsafe fn videoMaximumDuration(&self) -> NSTimeInterval;

        #[method(setVideoMaximumDuration:)]
        pub unsafe fn setVideoMaximumDuration(&self, video_maximum_duration: NSTimeInterval);

        #[cfg(feature = "UIImagePickerController")]
        #[method(videoQuality)]
        pub unsafe fn videoQuality(&self) -> UIImagePickerControllerQualityType;

        #[cfg(feature = "UIImagePickerController")]
        #[method(setVideoQuality:)]
        pub unsafe fn setVideoQuality(&self, video_quality: UIImagePickerControllerQualityType);
    }
);

extern_methods!(
    /// Methods declared on superclass `UINavigationController`
    #[cfg(all(
        feature = "UINavigationController",
        feature = "UIResponder",
        feature = "UIViewController"
    ))]
    unsafe impl UIVideoEditorController {
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
    unsafe impl UIVideoEditorController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait UIVideoEditorControllerDelegate:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(all(
            feature = "UINavigationController",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(videoEditorController:didSaveEditedVideoToPath:)]
        unsafe fn videoEditorController_didSaveEditedVideoToPath(
            &self,
            editor: &UIVideoEditorController,
            edited_video_path: &NSString,
        );

        #[cfg(all(
            feature = "UINavigationController",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(videoEditorController:didFailWithError:)]
        unsafe fn videoEditorController_didFailWithError(
            &self,
            editor: &UIVideoEditorController,
            error: &NSError,
        );

        #[cfg(all(
            feature = "UINavigationController",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(videoEditorControllerDidCancel:)]
        unsafe fn videoEditorControllerDidCancel(&self, editor: &UIVideoEditorController);
    }

    unsafe impl ProtocolType for dyn UIVideoEditorControllerDelegate {}
);
