//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UISceneConfiguration;

    unsafe impl ClassType for UISceneConfiguration {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSCoding for UISceneConfiguration {}

unsafe impl NSCopying for UISceneConfiguration {}

unsafe impl CopyingHelper for UISceneConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UISceneConfiguration {}

unsafe impl NSSecureCoding for UISceneConfiguration {}

extern_methods!(
    unsafe impl UISceneConfiguration {
        #[cfg(feature = "UISceneDefinitions")]
        #[method_id(@__retain_semantics Other configurationWithName:sessionRole:)]
        pub unsafe fn configurationWithName_sessionRole(
            name: Option<&NSString>,
            session_role: &UISceneSessionRole,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "UISceneDefinitions")]
        #[method_id(@__retain_semantics Init initWithName:sessionRole:)]
        pub unsafe fn initWithName_sessionRole(
            this: Allocated<Self>,
            name: Option<&NSString>,
            session_role: &UISceneSessionRole,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "UISceneDefinitions")]
        #[method_id(@__retain_semantics Other role)]
        pub unsafe fn role(&self) -> Retained<UISceneSessionRole>;

        #[method(sceneClass)]
        pub unsafe fn sceneClass(&self) -> Option<&'static AnyClass>;

        #[method(setSceneClass:)]
        pub unsafe fn setSceneClass(&self, scene_class: Option<&AnyClass>);

        #[method(delegateClass)]
        pub unsafe fn delegateClass(&self) -> Option<&'static AnyClass>;

        #[method(setDelegateClass:)]
        pub unsafe fn setDelegateClass(&self, delegate_class: Option<&AnyClass>);

        #[cfg(feature = "UIStoryboard")]
        #[method_id(@__retain_semantics Other storyboard)]
        pub unsafe fn storyboard(&self) -> Option<Retained<UIStoryboard>>;

        #[cfg(feature = "UIStoryboard")]
        #[method(setStoryboard:)]
        pub unsafe fn setStoryboard(&self, storyboard: Option<&UIStoryboard>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UISceneConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UISceneSession;

    unsafe impl ClassType for UISceneSession {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSCoding for UISceneSession {}

unsafe impl NSObjectProtocol for UISceneSession {}

unsafe impl NSSecureCoding for UISceneSession {}

extern_methods!(
    unsafe impl UISceneSession {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(feature = "UIResponder", feature = "UIScene"))]
        #[method_id(@__retain_semantics Other scene)]
        pub unsafe fn scene(&self) -> Option<Retained<UIScene>>;

        #[cfg(feature = "UISceneDefinitions")]
        #[method_id(@__retain_semantics Other role)]
        pub unsafe fn role(&self) -> Retained<UISceneSessionRole>;

        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration(&self) -> Retained<UISceneConfiguration>;

        #[method_id(@__retain_semantics Other persistentIdentifier)]
        pub unsafe fn persistentIdentifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other stateRestorationActivity)]
        pub unsafe fn stateRestorationActivity(&self) -> Option<Retained<NSUserActivity>>;

        #[method(setStateRestorationActivity:)]
        pub unsafe fn setStateRestorationActivity(
            &self,
            state_restoration_activity: Option<&NSUserActivity>,
        );

        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&NSDictionary<NSString, AnyObject>>);
    }
);
