//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct LAEnvironment;
);

unsafe impl NSObjectProtocol for LAEnvironment {}

extern_methods!(
    unsafe impl LAEnvironment {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(addObserver:)]
        pub unsafe fn addObserver(&self, observer: &ProtocolObject<dyn LAEnvironmentObserver>);

        #[method(removeObserver:)]
        pub unsafe fn removeObserver(&self, observer: &ProtocolObject<dyn LAEnvironmentObserver>);

        #[method_id(@__retain_semantics Other currentUser)]
        pub unsafe fn currentUser() -> Retained<LAEnvironment>;

        #[cfg(feature = "LAEnvironmentState")]
        #[method_id(@__retain_semantics Other state)]
        pub unsafe fn state(&self) -> Retained<LAEnvironmentState>;
    }
);

extern_protocol!(
    pub unsafe trait LAEnvironmentObserver: NSObjectProtocol {
        #[cfg(feature = "LAEnvironmentState")]
        #[optional]
        #[method(environment:stateDidChangeFromOldState:)]
        unsafe fn environment_stateDidChangeFromOldState(
            &self,
            environment: &LAEnvironment,
            old_state: &LAEnvironmentState,
        );
    }

    unsafe impl ProtocolType for dyn LAEnvironmentObserver {}
);
