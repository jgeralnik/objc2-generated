//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AMAction")]
    pub struct AMBundleAction;

    #[cfg(feature = "AMAction")]
    unsafe impl ClassType for AMBundleAction {
        #[inherits(NSObject)]
        type Super = AMAction;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AMAction")]
unsafe impl NSCoding for AMBundleAction {}

#[cfg(feature = "AMAction")]
unsafe impl NSCopying for AMBundleAction {}

#[cfg(feature = "AMAction")]
unsafe impl CopyingHelper for AMBundleAction {
    type Result = Self;
}

#[cfg(feature = "AMAction")]
unsafe impl NSObjectProtocol for AMBundleAction {}

#[cfg(feature = "AMAction")]
unsafe impl NSSecureCoding for AMBundleAction {}

extern_methods!(
    #[cfg(feature = "AMAction")]
    unsafe impl AMBundleAction {
        #[method(awakeFromBundle)]
        pub unsafe fn awakeFromBundle(&self);

        #[method(hasView)]
        pub unsafe fn hasView(&self) -> bool;

        #[cfg(feature = "objc2-app-kit")]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self, mtm: MainThreadMarker) -> Option<Retained<NSView>>;

        #[method_id(@__retain_semantics Other bundle)]
        pub unsafe fn bundle(&self) -> Retained<NSBundle>;

        #[method_id(@__retain_semantics Other parameters)]
        pub unsafe fn parameters(
            &self,
        ) -> Option<Retained<NSMutableDictionary<NSString, AnyObject>>>;

        #[method(setParameters:)]
        pub unsafe fn setParameters(
            &self,
            parameters: Option<&NSMutableDictionary<NSString, AnyObject>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `AMAction`
    #[cfg(feature = "AMAction")]
    unsafe impl AMBundleAction {
        #[method_id(@__retain_semantics Init initWithDefinition:fromArchive:)]
        pub unsafe fn initWithDefinition_fromArchive(
            this: Allocated<Self>,
            dict: Option<&NSDictionary<NSString, AnyObject>>,
            archived: bool,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithContentsOfURL:error:_)]
        pub unsafe fn initWithContentsOfURL_error(
            this: Allocated<Self>,
            file_url: &NSURL,
        ) -> Result<Retained<Self>, Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AMAction")]
    unsafe impl AMBundleAction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
