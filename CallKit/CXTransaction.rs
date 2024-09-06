//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CXTransaction;

    unsafe impl ClassType for CXTransaction {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for CXTransaction {}

unsafe impl NSCopying for CXTransaction {}

unsafe impl CopyingHelper for CXTransaction {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CXTransaction {}

unsafe impl NSSecureCoding for CXTransaction {}

extern_methods!(
    unsafe impl CXTransaction {
        #[method_id(@__retain_semantics Other UUID)]
        pub unsafe fn UUID(&self) -> Retained<NSUUID>;

        #[method(isComplete)]
        pub unsafe fn isComplete(&self) -> bool;

        #[cfg(feature = "CXAction")]
        #[method_id(@__retain_semantics Other actions)]
        pub unsafe fn actions(&self) -> Retained<NSArray<CXAction>>;

        #[cfg(feature = "CXAction")]
        #[method_id(@__retain_semantics Init initWithActions:)]
        pub unsafe fn initWithActions(
            this: Allocated<Self>,
            actions: &NSArray<CXAction>,
        ) -> Retained<Self>;

        #[cfg(feature = "CXAction")]
        #[method_id(@__retain_semantics Init initWithAction:)]
        pub unsafe fn initWithAction(this: Allocated<Self>, action: &CXAction) -> Retained<Self>;

        #[cfg(feature = "CXAction")]
        #[method(addAction:)]
        pub unsafe fn addAction(&self, action: &CXAction);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CXTransaction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
