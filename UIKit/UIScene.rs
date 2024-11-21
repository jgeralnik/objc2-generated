//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIResponder")]
    pub struct UIScene;
);

#[cfg(feature = "UIResponder")]
unsafe impl NSObjectProtocol for UIScene {}

#[cfg(feature = "UIResponder")]
unsafe impl UIResponderStandardEditActions for UIScene {}

extern_methods!(
    #[cfg(feature = "UIResponder")]
    unsafe impl UIScene {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(feature = "UISceneOptions", feature = "UISceneSession"))]
        #[method_id(@__retain_semantics Init initWithSession:connectionOptions:)]
        pub unsafe fn initWithSession_connectionOptions(
            this: Allocated<Self>,
            session: &UISceneSession,
            connection_options: &UISceneConnectionOptions,
        ) -> Retained<Self>;

        #[cfg(feature = "UISceneSession")]
        #[method_id(@__retain_semantics Other session)]
        pub unsafe fn session(&self) -> Retained<UISceneSession>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn UISceneDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn UISceneDelegate>>);

        #[cfg(feature = "UISceneDefinitions")]
        #[method(activationState)]
        pub unsafe fn activationState(&self) -> UISceneActivationState;

        #[cfg(all(feature = "UISceneOptions", feature = "block2"))]
        #[method(openURL:options:completionHandler:)]
        pub unsafe fn openURL_options_completionHandler(
            &self,
            url: &NSURL,
            options: Option<&UISceneOpenExternalURLOptions>,
            completion: Option<&block2::Block<dyn Fn(Bool)>>,
        );

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[method_id(@__retain_semantics Other subtitle)]
        pub unsafe fn subtitle(&self) -> Retained<NSString>;

        #[method(setSubtitle:)]
        pub unsafe fn setSubtitle(&self, subtitle: &NSString);

        #[cfg(feature = "UISceneActivationConditions")]
        #[method_id(@__retain_semantics Other activationConditions)]
        pub unsafe fn activationConditions(&self) -> Retained<UISceneActivationConditions>;

        #[cfg(feature = "UISceneActivationConditions")]
        #[method(setActivationConditions:)]
        pub unsafe fn setActivationConditions(
            &self,
            activation_conditions: &UISceneActivationConditions,
        );
    }
);

extern_protocol!(
    pub unsafe trait UISceneDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(
            feature = "UIResponder",
            feature = "UISceneOptions",
            feature = "UISceneSession"
        ))]
        #[optional]
        #[method(scene:willConnectToSession:options:)]
        unsafe fn scene_willConnectToSession_options(
            &self,
            scene: &UIScene,
            session: &UISceneSession,
            connection_options: &UISceneConnectionOptions,
        );

        #[cfg(feature = "UIResponder")]
        #[optional]
        #[method(sceneDidDisconnect:)]
        unsafe fn sceneDidDisconnect(&self, scene: &UIScene);

        #[cfg(feature = "UIResponder")]
        #[optional]
        #[method(sceneDidBecomeActive:)]
        unsafe fn sceneDidBecomeActive(&self, scene: &UIScene);

        #[cfg(feature = "UIResponder")]
        #[optional]
        #[method(sceneWillResignActive:)]
        unsafe fn sceneWillResignActive(&self, scene: &UIScene);

        #[cfg(feature = "UIResponder")]
        #[optional]
        #[method(sceneWillEnterForeground:)]
        unsafe fn sceneWillEnterForeground(&self, scene: &UIScene);

        #[cfg(feature = "UIResponder")]
        #[optional]
        #[method(sceneDidEnterBackground:)]
        unsafe fn sceneDidEnterBackground(&self, scene: &UIScene);

        #[cfg(all(feature = "UIOpenURLContext", feature = "UIResponder"))]
        #[optional]
        #[method(scene:openURLContexts:)]
        unsafe fn scene_openURLContexts(
            &self,
            scene: &UIScene,
            url_contexts: &NSSet<UIOpenURLContext>,
        );

        #[cfg(feature = "UIResponder")]
        #[optional]
        #[method_id(@__retain_semantics Other stateRestorationActivityForScene:)]
        unsafe fn stateRestorationActivityForScene(
            &self,
            scene: &UIScene,
        ) -> Option<Retained<NSUserActivity>>;

        #[cfg(feature = "UIResponder")]
        #[optional]
        #[method(scene:restoreInteractionStateWithUserActivity:)]
        unsafe fn scene_restoreInteractionStateWithUserActivity(
            &self,
            scene: &UIScene,
            state_restoration_activity: &NSUserActivity,
        );

        #[cfg(feature = "UIResponder")]
        #[optional]
        #[method(scene:willContinueUserActivityWithType:)]
        unsafe fn scene_willContinueUserActivityWithType(
            &self,
            scene: &UIScene,
            user_activity_type: &NSString,
        );

        #[cfg(feature = "UIResponder")]
        #[optional]
        #[method(scene:continueUserActivity:)]
        unsafe fn scene_continueUserActivity(
            &self,
            scene: &UIScene,
            user_activity: &NSUserActivity,
        );

        #[cfg(feature = "UIResponder")]
        #[optional]
        #[method(scene:didFailToContinueUserActivityWithType:error:)]
        unsafe fn scene_didFailToContinueUserActivityWithType_error(
            &self,
            scene: &UIScene,
            user_activity_type: &NSString,
            error: &NSError,
        );

        #[cfg(feature = "UIResponder")]
        #[optional]
        #[method(scene:didUpdateUserActivity:)]
        unsafe fn scene_didUpdateUserActivity(
            &self,
            scene: &UIScene,
            user_activity: &NSUserActivity,
        );
    }

    unsafe impl ProtocolType for dyn UISceneDelegate {}
);

extern "C" {
    pub static UISceneWillConnectNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UISceneDidDisconnectNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UISceneDidActivateNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UISceneWillDeactivateNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UISceneWillEnterForegroundNotification: &'static NSNotificationName;
}

extern "C" {
    pub static UISceneDidEnterBackgroundNotification: &'static NSNotificationName;
}

extern "C" {
    #[cfg(feature = "UISceneDefinitions")]
    pub static UISceneSessionRoleImmersiveSpaceApplication: &'static UISceneSessionRole;
}
