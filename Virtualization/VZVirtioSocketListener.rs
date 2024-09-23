//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZVirtioSocketListener;

    unsafe impl ClassType for VZVirtioSocketListener {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for VZVirtioSocketListener {}

extern_methods!(
    unsafe impl VZVirtioSocketListener {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn VZVirtioSocketListenerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn VZVirtioSocketListenerDelegate>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl VZVirtioSocketListener {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait VZVirtioSocketListenerDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "VZSocketDevice",
            feature = "VZVirtioSocketConnection",
            feature = "VZVirtioSocketDevice"
        ))]
        #[optional]
        #[method(listener:shouldAcceptNewConnection:fromSocketDevice:)]
        unsafe fn listener_shouldAcceptNewConnection_fromSocketDevice(
            &self,
            listener: &VZVirtioSocketListener,
            connection: &VZVirtioSocketConnection,
            socket_device: &VZVirtioSocketDevice,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn VZVirtioSocketListenerDelegate {}
);
