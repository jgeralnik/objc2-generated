//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKSavedGame;

    unsafe impl ClassType for GKSavedGame {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for GKSavedGame {}

unsafe impl NSObjectProtocol for GKSavedGame {}

extern_methods!(
    unsafe impl GKSavedGame {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other deviceName)]
        pub unsafe fn deviceName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other modificationDate)]
        pub unsafe fn modificationDate(&self) -> Option<Id<NSDate>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method(loadDataWithCompletionHandler:)]
        pub unsafe fn loadDataWithCompletionHandler(
            &self,
            handler: Option<&Block<dyn Fn(*mut NSData, *mut NSError)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GKSavedGame {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// GKSavedGame
    #[cfg(all(
        feature = "GameKit_GKBasePlayer",
        feature = "GameKit_GKLocalPlayer",
        feature = "GameKit_GKPlayer"
    ))]
    unsafe impl GKLocalPlayer {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[method(fetchSavedGamesWithCompletionHandler:)]
        pub unsafe fn fetchSavedGamesWithCompletionHandler(
            &self,
            handler: Option<&Block<dyn Fn(*mut NSArray<GKSavedGame>, *mut NSError)>>,
        );

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(saveGameData:withName:completionHandler:)]
        pub unsafe fn saveGameData_withName_completionHandler(
            &self,
            data: &NSData,
            name: &NSString,
            handler: Option<&Block<dyn Fn(*mut GKSavedGame, *mut NSError)>>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(deleteSavedGamesWithName:completionHandler:)]
        pub unsafe fn deleteSavedGamesWithName_completionHandler(
            &self,
            name: &NSString,
            handler: Option<&Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSData",
            feature = "Foundation_NSError"
        ))]
        #[method(resolveConflictingSavedGames:withData:completionHandler:)]
        pub unsafe fn resolveConflictingSavedGames_withData_completionHandler(
            &self,
            conflicting_saved_games: &NSArray<GKSavedGame>,
            data: &NSData,
            handler: Option<&Block<dyn Fn(*mut NSArray<GKSavedGame>, *mut NSError)>>,
        );
    }
);

#[cfg(all(
    feature = "GameKit_GKBasePlayer",
    feature = "GameKit_GKLocalPlayer",
    feature = "GameKit_GKPlayer",
    feature = "GameKit_GKSavedGameListener"
))]
unsafe impl GKSavedGameListener for GKLocalPlayer {}
