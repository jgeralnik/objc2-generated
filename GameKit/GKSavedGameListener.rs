//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamekit/gksavedgamelistener?language=objc)
    pub unsafe trait GKSavedGameListener: NSObjectProtocol {
        #[cfg(all(
            feature = "GKBasePlayer",
            feature = "GKPlayer",
            feature = "GKSavedGame"
        ))]
        #[optional]
        #[method(player:didModifySavedGame:)]
        unsafe fn player_didModifySavedGame(&self, player: &GKPlayer, saved_game: &GKSavedGame);

        #[cfg(all(
            feature = "GKBasePlayer",
            feature = "GKPlayer",
            feature = "GKSavedGame"
        ))]
        #[optional]
        #[method(player:hasConflictingSavedGames:)]
        unsafe fn player_hasConflictingSavedGames(
            &self,
            player: &GKPlayer,
            saved_games: &NSArray<GKSavedGame>,
        );
    }

    unsafe impl ProtocolType for dyn GKSavedGameListener {}
);
