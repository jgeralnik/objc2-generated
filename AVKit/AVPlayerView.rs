//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVPlayerViewControlsStyle(pub NSInteger);
impl AVPlayerViewControlsStyle {
    #[doc(alias = "AVPlayerViewControlsStyleNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "AVPlayerViewControlsStyleInline")]
    pub const Inline: Self = Self(1);
    #[doc(alias = "AVPlayerViewControlsStyleFloating")]
    pub const Floating: Self = Self(2);
    #[doc(alias = "AVPlayerViewControlsStyleMinimal")]
    pub const Minimal: Self = Self(3);
    #[doc(alias = "AVPlayerViewControlsStyleDefault")]
    pub const Default: Self = Self(AVPlayerViewControlsStyle::Inline.0);
}

unsafe impl Encode for AVPlayerViewControlsStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVPlayerViewControlsStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    pub struct AVPlayerView;

    #[cfg(feature = "objc2-app-kit")]
    unsafe impl ClassType for AVPlayerView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSAccessibility for AVPlayerView {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSAccessibilityElementProtocol for AVPlayerView {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSAnimatablePropertyContainer for AVPlayerView {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSAppearanceCustomization for AVPlayerView {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSCoding for AVPlayerView {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSDraggingDestination for AVPlayerView {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSObjectProtocol for AVPlayerView {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSUserInterfaceItemIdentification for AVPlayerView {}

extern_methods!(
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl AVPlayerView {
        #[method(controlsStyle)]
        pub unsafe fn controlsStyle(&self) -> AVPlayerViewControlsStyle;

        #[method(setControlsStyle:)]
        pub unsafe fn setControlsStyle(&self, controls_style: AVPlayerViewControlsStyle);

        #[method(isReadyForDisplay)]
        pub unsafe fn isReadyForDisplay(&self) -> bool;

        #[method(videoBounds)]
        pub unsafe fn videoBounds(&self) -> NSRect;

        #[method_id(@__retain_semantics Other contentOverlayView)]
        pub unsafe fn contentOverlayView(&self) -> Option<Id<NSView>>;

        #[method(updatesNowPlayingInfoCenter)]
        pub unsafe fn updatesNowPlayingInfoCenter(&self) -> bool;

        #[method(setUpdatesNowPlayingInfoCenter:)]
        pub unsafe fn setUpdatesNowPlayingInfoCenter(&self, updates_now_playing_info_center: bool);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn AVPlayerViewDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn AVPlayerViewDelegate>>,
        );

        #[cfg(feature = "AVPlaybackSpeed")]
        #[method_id(@__retain_semantics Other speeds)]
        pub unsafe fn speeds(&self) -> Id<NSArray<AVPlaybackSpeed>>;

        #[cfg(feature = "AVPlaybackSpeed")]
        #[method(setSpeeds:)]
        pub unsafe fn setSpeeds(&self, speeds: &NSArray<AVPlaybackSpeed>);

        #[cfg(feature = "AVPlaybackSpeed")]
        #[method_id(@__retain_semantics Other selectedSpeed)]
        pub unsafe fn selectedSpeed(&self) -> Option<Id<AVPlaybackSpeed>>;

        #[cfg(feature = "AVPlaybackSpeed")]
        #[method(selectSpeed:)]
        pub unsafe fn selectSpeed(&self, speed: &AVPlaybackSpeed);

        #[method(allowsVideoFrameAnalysis)]
        pub unsafe fn allowsVideoFrameAnalysis(&self) -> bool;

        #[method(setAllowsVideoFrameAnalysis:)]
        pub unsafe fn setAllowsVideoFrameAnalysis(&self, allows_video_frame_analysis: bool);

        #[cfg(feature = "AVKitTypes")]
        #[method(videoFrameAnalysisTypes)]
        pub unsafe fn videoFrameAnalysisTypes(&self) -> AVVideoFrameAnalysisType;

        #[cfg(feature = "AVKitTypes")]
        #[method(setVideoFrameAnalysisTypes:)]
        pub unsafe fn setVideoFrameAnalysisTypes(
            &self,
            video_frame_analysis_types: AVVideoFrameAnalysisType,
        );

        #[method(allowsMagnification)]
        pub unsafe fn allowsMagnification(&self) -> bool;

        #[method(setAllowsMagnification:)]
        pub unsafe fn setAllowsMagnification(&self, allows_magnification: bool);

        #[method(magnification)]
        pub unsafe fn magnification(&self) -> CGFloat;

        #[method(setMagnification:)]
        pub unsafe fn setMagnification(&self, magnification: CGFloat);

        #[method(setMagnification:centeredAtPoint:)]
        pub unsafe fn setMagnification_centeredAtPoint(
            &self,
            magnification: CGFloat,
            point: CGPoint,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl AVPlayerView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl AVPlayerView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl AVPlayerView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_methods!(
    /// AVPlayerViewCustomization
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl AVPlayerView {
        #[method(showsFrameSteppingButtons)]
        pub unsafe fn showsFrameSteppingButtons(&self) -> bool;

        #[method(setShowsFrameSteppingButtons:)]
        pub unsafe fn setShowsFrameSteppingButtons(&self, shows_frame_stepping_buttons: bool);

        #[method(showsSharingServiceButton)]
        pub unsafe fn showsSharingServiceButton(&self) -> bool;

        #[method(setShowsSharingServiceButton:)]
        pub unsafe fn setShowsSharingServiceButton(&self, shows_sharing_service_button: bool);

        #[method_id(@__retain_semantics Other actionPopUpButtonMenu)]
        pub unsafe fn actionPopUpButtonMenu(&self) -> Option<Id<NSMenu>>;

        #[method(setActionPopUpButtonMenu:)]
        pub unsafe fn setActionPopUpButtonMenu(&self, action_pop_up_button_menu: Option<&NSMenu>);

        #[method(showsFullScreenToggleButton)]
        pub unsafe fn showsFullScreenToggleButton(&self) -> bool;

        #[method(setShowsFullScreenToggleButton:)]
        pub unsafe fn setShowsFullScreenToggleButton(&self, shows_full_screen_toggle_button: bool);

        #[method(showsTimecodes)]
        pub unsafe fn showsTimecodes(&self) -> bool;

        #[method(setShowsTimecodes:)]
        pub unsafe fn setShowsTimecodes(&self, shows_timecodes: bool);
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVPlayerViewTrimResult(pub NSInteger);
impl AVPlayerViewTrimResult {
    pub const AVPlayerViewTrimOKButton: Self = Self(0);
    pub const AVPlayerViewTrimCancelButton: Self = Self(1);
}

unsafe impl Encode for AVPlayerViewTrimResult {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVPlayerViewTrimResult {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// AVPlayerViewTrimming
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl AVPlayerView {
        #[method(canBeginTrimming)]
        pub unsafe fn canBeginTrimming(&self) -> bool;

        #[cfg(feature = "block2")]
        #[method(beginTrimmingWithCompletionHandler:)]
        pub unsafe fn beginTrimmingWithCompletionHandler(
            &self,
            handler: Option<&block2::Block<dyn Fn(AVPlayerViewTrimResult)>>,
        );
    }
);

extern_methods!(
    /// AVPlayerViewChapterIndicator
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl AVPlayerView {
        #[method(flashChapterNumber:chapterTitle:)]
        pub unsafe fn flashChapterNumber_chapterTitle(
            &self,
            chapter_number: NSUInteger,
            chapter_title: Option<&NSString>,
        );
    }
);

extern_methods!(
    /// AVPlayerViewPictureInPictureSupport
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl AVPlayerView {
        #[method(allowsPictureInPicturePlayback)]
        pub unsafe fn allowsPictureInPicturePlayback(&self) -> bool;

        #[method(setAllowsPictureInPicturePlayback:)]
        pub unsafe fn setAllowsPictureInPicturePlayback(
            &self,
            allows_picture_in_picture_playback: bool,
        );

        #[method_id(@__retain_semantics Other pictureInPictureDelegate)]
        pub unsafe fn pictureInPictureDelegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn AVPlayerViewPictureInPictureDelegate>>>;

        #[method(setPictureInPictureDelegate:)]
        pub unsafe fn setPictureInPictureDelegate(
            &self,
            picture_in_picture_delegate: Option<
                &ProtocolObject<dyn AVPlayerViewPictureInPictureDelegate>,
            >,
        );
    }
);

extern_protocol!(
    pub unsafe trait AVPlayerViewDelegate: NSObjectProtocol {
        #[cfg(feature = "objc2-app-kit")]
        #[optional]
        #[method(playerViewWillEnterFullScreen:)]
        unsafe fn playerViewWillEnterFullScreen(&self, player_view: &AVPlayerView);

        #[cfg(feature = "objc2-app-kit")]
        #[optional]
        #[method(playerViewDidEnterFullScreen:)]
        unsafe fn playerViewDidEnterFullScreen(&self, player_view: &AVPlayerView);

        #[cfg(feature = "objc2-app-kit")]
        #[optional]
        #[method(playerViewWillExitFullScreen:)]
        unsafe fn playerViewWillExitFullScreen(&self, player_view: &AVPlayerView);

        #[cfg(feature = "objc2-app-kit")]
        #[optional]
        #[method(playerViewDidExitFullScreen:)]
        unsafe fn playerViewDidExitFullScreen(&self, player_view: &AVPlayerView);

        #[cfg(all(feature = "block2", feature = "objc2-app-kit"))]
        #[optional]
        #[method(playerView:restoreUserInterfaceForFullScreenExitWithCompletionHandler:)]
        unsafe fn playerView_restoreUserInterfaceForFullScreenExitWithCompletionHandler(
            &self,
            player_view: &AVPlayerView,
            completion_handler: &block2::Block<dyn Fn(Bool)>,
        );
    }

    unsafe impl ProtocolType for dyn AVPlayerViewDelegate {}
);

extern_protocol!(
    pub unsafe trait AVPlayerViewPictureInPictureDelegate: NSObjectProtocol {
        #[cfg(feature = "objc2-app-kit")]
        #[optional]
        #[method(playerViewWillStartPictureInPicture:)]
        unsafe fn playerViewWillStartPictureInPicture(&self, player_view: &AVPlayerView);

        #[cfg(feature = "objc2-app-kit")]
        #[optional]
        #[method(playerViewDidStartPictureInPicture:)]
        unsafe fn playerViewDidStartPictureInPicture(&self, player_view: &AVPlayerView);

        #[cfg(feature = "objc2-app-kit")]
        #[optional]
        #[method(playerView:failedToStartPictureInPictureWithError:)]
        unsafe fn playerView_failedToStartPictureInPictureWithError(
            &self,
            player_view: &AVPlayerView,
            error: &NSError,
        );

        #[cfg(feature = "objc2-app-kit")]
        #[optional]
        #[method(playerViewWillStopPictureInPicture:)]
        unsafe fn playerViewWillStopPictureInPicture(&self, player_view: &AVPlayerView);

        #[cfg(feature = "objc2-app-kit")]
        #[optional]
        #[method(playerViewDidStopPictureInPicture:)]
        unsafe fn playerViewDidStopPictureInPicture(&self, player_view: &AVPlayerView);

        #[cfg(all(feature = "block2", feature = "objc2-app-kit"))]
        #[optional]
        #[method(playerView:restoreUserInterfaceForPictureInPictureStopWithCompletionHandler:)]
        unsafe fn playerView_restoreUserInterfaceForPictureInPictureStopWithCompletionHandler(
            &self,
            player_view: &AVPlayerView,
            completion_handler: &block2::Block<dyn Fn(Bool)>,
        );

        #[cfg(feature = "objc2-app-kit")]
        #[optional]
        #[method(playerViewShouldAutomaticallyDismissAtPictureInPictureStart:)]
        unsafe fn playerViewShouldAutomaticallyDismissAtPictureInPictureStart(
            &self,
            player_view: &AVPlayerView,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn AVPlayerViewPictureInPictureDelegate {}
);