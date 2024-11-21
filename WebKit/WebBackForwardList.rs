//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct WebBackForwardList;
);

unsafe impl NSObjectProtocol for WebBackForwardList {}

extern_methods!(
    unsafe impl WebBackForwardList {
        #[cfg(feature = "WebHistoryItem")]
        #[deprecated]
        #[method(addItem:)]
        pub unsafe fn addItem(&self, item: Option<&WebHistoryItem>);

        #[deprecated]
        #[method(goBack)]
        pub unsafe fn goBack(&self);

        #[deprecated]
        #[method(goForward)]
        pub unsafe fn goForward(&self);

        #[cfg(feature = "WebHistoryItem")]
        #[deprecated]
        #[method(goToItem:)]
        pub unsafe fn goToItem(&self, item: Option<&WebHistoryItem>);

        #[cfg(feature = "WebHistoryItem")]
        #[deprecated]
        #[method_id(@__retain_semantics Other backItem)]
        pub unsafe fn backItem(&self) -> Option<Retained<WebHistoryItem>>;

        #[cfg(feature = "WebHistoryItem")]
        #[deprecated]
        #[method_id(@__retain_semantics Other currentItem)]
        pub unsafe fn currentItem(&self) -> Option<Retained<WebHistoryItem>>;

        #[cfg(feature = "WebHistoryItem")]
        #[deprecated]
        #[method_id(@__retain_semantics Other forwardItem)]
        pub unsafe fn forwardItem(&self) -> Option<Retained<WebHistoryItem>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other backListWithLimit:)]
        pub unsafe fn backListWithLimit(&self, limit: c_int) -> Option<Retained<NSArray>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other forwardListWithLimit:)]
        pub unsafe fn forwardListWithLimit(&self, limit: c_int) -> Option<Retained<NSArray>>;

        #[deprecated]
        #[method(capacity)]
        pub unsafe fn capacity(&self) -> c_int;

        #[deprecated]
        #[method(setCapacity:)]
        pub unsafe fn setCapacity(&self, capacity: c_int);

        #[deprecated]
        #[method(backListCount)]
        pub unsafe fn backListCount(&self) -> c_int;

        #[deprecated]
        #[method(forwardListCount)]
        pub unsafe fn forwardListCount(&self) -> c_int;

        #[cfg(feature = "WebHistoryItem")]
        #[deprecated]
        #[method(containsItem:)]
        pub unsafe fn containsItem(&self, item: Option<&WebHistoryItem>) -> bool;

        #[cfg(feature = "WebHistoryItem")]
        #[deprecated]
        #[method_id(@__retain_semantics Other itemAtIndex:)]
        pub unsafe fn itemAtIndex(&self, index: c_int) -> Option<Retained<WebHistoryItem>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WebBackForwardList {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// WebBackForwardListDeprecated
    unsafe impl WebBackForwardList {
        #[deprecated]
        #[method(setPageCacheSize:)]
        pub unsafe fn setPageCacheSize(&self, size: NSUInteger);

        #[deprecated]
        #[method(pageCacheSize)]
        pub unsafe fn pageCacheSize(&self) -> NSUInteger;
    }
);
