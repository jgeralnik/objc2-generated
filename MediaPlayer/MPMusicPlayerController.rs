//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MPMusicPlaybackState {
        #[doc(alias = "MPMusicPlaybackStateStopped")]
        Stopped = 0,
        #[doc(alias = "MPMusicPlaybackStatePlaying")]
        Playing = 1,
        #[doc(alias = "MPMusicPlaybackStatePaused")]
        Paused = 2,
        #[doc(alias = "MPMusicPlaybackStateInterrupted")]
        Interrupted = 3,
        #[doc(alias = "MPMusicPlaybackStateSeekingForward")]
        SeekingForward = 4,
        #[doc(alias = "MPMusicPlaybackStateSeekingBackward")]
        SeekingBackward = 5,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MPMusicRepeatMode {
        #[doc(alias = "MPMusicRepeatModeDefault")]
        Default = 0,
        #[doc(alias = "MPMusicRepeatModeNone")]
        None = 1,
        #[doc(alias = "MPMusicRepeatModeOne")]
        One = 2,
        #[doc(alias = "MPMusicRepeatModeAll")]
        All = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MPMusicShuffleMode {
        #[doc(alias = "MPMusicShuffleModeDefault")]
        Default = 0,
        #[doc(alias = "MPMusicShuffleModeOff")]
        Off = 1,
        #[doc(alias = "MPMusicShuffleModeSongs")]
        Songs = 2,
        #[doc(alias = "MPMusicShuffleModeAlbums")]
        Albums = 3,
    }
);

extern_protocol!(
    pub unsafe trait MPSystemMusicPlayerController: NSObjectProtocol {
        #[cfg(feature = "MediaPlayer_MPMusicPlayerQueueDescriptor")]
        #[method(openToPlayQueueDescriptor:)]
        unsafe fn openToPlayQueueDescriptor(&self, queue_descriptor: &MPMusicPlayerQueueDescriptor);
    }

    unsafe impl ProtocolType for dyn MPSystemMusicPlayerController {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPMusicPlayerController;

    unsafe impl ClassType for MPMusicPlayerController {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPMediaPlayback")]
unsafe impl MPMediaPlayback for MPMusicPlayerController {}

unsafe impl NSObjectProtocol for MPMusicPlayerController {}

extern_methods!(
    unsafe impl MPMusicPlayerController {
        #[method_id(@__retain_semantics Other applicationMusicPlayer)]
        pub unsafe fn applicationMusicPlayer() -> Id<MPMusicPlayerController>;

        #[cfg(feature = "MediaPlayer_MPMusicPlayerApplicationController")]
        #[method_id(@__retain_semantics Other applicationQueuePlayer)]
        pub unsafe fn applicationQueuePlayer() -> Id<MPMusicPlayerApplicationController>;

        #[method_id(@__retain_semantics Other systemMusicPlayer)]
        pub unsafe fn systemMusicPlayer() -> Id<MPMusicPlayerController>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method(playbackState)]
        pub unsafe fn playbackState(&self) -> MPMusicPlaybackState;

        #[method(repeatMode)]
        pub unsafe fn repeatMode(&self) -> MPMusicRepeatMode;

        #[method(setRepeatMode:)]
        pub unsafe fn setRepeatMode(&self, repeat_mode: MPMusicRepeatMode);

        #[method(shuffleMode)]
        pub unsafe fn shuffleMode(&self) -> MPMusicShuffleMode;

        #[method(setShuffleMode:)]
        pub unsafe fn setShuffleMode(&self, shuffle_mode: MPMusicShuffleMode);

        #[deprecated = "Use MPVolumeView for volume control."]
        #[method(volume)]
        pub unsafe fn volume(&self) -> c_float;

        #[deprecated = "Use MPVolumeView for volume control."]
        #[method(setVolume:)]
        pub unsafe fn setVolume(&self, volume: c_float);

        #[cfg(all(
            feature = "MediaPlayer_MPMediaEntity",
            feature = "MediaPlayer_MPMediaItem"
        ))]
        #[method_id(@__retain_semantics Other nowPlayingItem)]
        pub unsafe fn nowPlayingItem(&self) -> Option<Id<MPMediaItem>>;

        #[cfg(all(
            feature = "MediaPlayer_MPMediaEntity",
            feature = "MediaPlayer_MPMediaItem"
        ))]
        #[method(setNowPlayingItem:)]
        pub unsafe fn setNowPlayingItem(&self, now_playing_item: Option<&MPMediaItem>);

        #[method(indexOfNowPlayingItem)]
        pub unsafe fn indexOfNowPlayingItem(&self) -> NSUInteger;

        #[cfg(feature = "MediaPlayer_MPMediaQuery")]
        #[method(setQueueWithQuery:)]
        pub unsafe fn setQueueWithQuery(&self, query: &MPMediaQuery);

        #[cfg(all(
            feature = "MediaPlayer_MPMediaEntity",
            feature = "MediaPlayer_MPMediaItemCollection"
        ))]
        #[method(setQueueWithItemCollection:)]
        pub unsafe fn setQueueWithItemCollection(&self, item_collection: &MPMediaItemCollection);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setQueueWithStoreIDs:)]
        pub unsafe fn setQueueWithStoreIDs(&self, store_i_ds: &NSArray<NSString>);

        #[cfg(feature = "MediaPlayer_MPMusicPlayerQueueDescriptor")]
        #[method(setQueueWithDescriptor:)]
        pub unsafe fn setQueueWithDescriptor(&self, descriptor: &MPMusicPlayerQueueDescriptor);

        #[cfg(feature = "MediaPlayer_MPMusicPlayerQueueDescriptor")]
        #[method(prependQueueDescriptor:)]
        pub unsafe fn prependQueueDescriptor(&self, descriptor: &MPMusicPlayerQueueDescriptor);

        #[cfg(feature = "MediaPlayer_MPMusicPlayerQueueDescriptor")]
        #[method(appendQueueDescriptor:)]
        pub unsafe fn appendQueueDescriptor(&self, descriptor: &MPMusicPlayerQueueDescriptor);

        #[cfg(feature = "Foundation_NSError")]
        #[method(prepareToPlayWithCompletionHandler:)]
        pub unsafe fn prepareToPlayWithCompletionHandler(
            &self,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[method(skipToNextItem)]
        pub unsafe fn skipToNextItem(&self);

        #[method(skipToBeginning)]
        pub unsafe fn skipToBeginning(&self);

        #[method(skipToPreviousItem)]
        pub unsafe fn skipToPreviousItem(&self);

        #[method(beginGeneratingPlaybackNotifications)]
        pub unsafe fn beginGeneratingPlaybackNotifications(&self);

        #[method(endGeneratingPlaybackNotifications)]
        pub unsafe fn endGeneratingPlaybackNotifications(&self);

        #[deprecated]
        #[method_id(@__retain_semantics Other iPodMusicPlayer)]
        pub unsafe fn iPodMusicPlayer() -> Id<MPMusicPlayerController>;
    }
);

extern "C" {
    #[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
    pub static MPMusicPlayerControllerPlaybackStateDidChangeNotification:
        &'static NSNotificationName;
}

extern "C" {
    #[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
    pub static MPMusicPlayerControllerNowPlayingItemDidChangeNotification:
        &'static NSNotificationName;
}

extern "C" {
    #[cfg(all(feature = "Foundation_NSNotification", feature = "Foundation_NSString"))]
    pub static MPMusicPlayerControllerVolumeDidChangeNotification: &'static NSNotificationName;
}
