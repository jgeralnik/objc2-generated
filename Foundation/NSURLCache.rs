//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSURLCacheStoragePolicy(pub NSUInteger);
impl NSURLCacheStoragePolicy {
    pub const NSURLCacheStorageAllowed: Self = Self(0);
    pub const NSURLCacheStorageAllowedInMemoryOnly: Self = Self(1);
    pub const NSURLCacheStorageNotAllowed: Self = Self(2);
}

unsafe impl Encode for NSURLCacheStoragePolicy {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSURLCacheStoragePolicy {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCachedURLResponse;

    unsafe impl ClassType for NSCachedURLResponse {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for NSCachedURLResponse {}

unsafe impl Sync for NSCachedURLResponse {}

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSCachedURLResponse {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSCachedURLResponse {}

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSCachedURLResponse {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSCachedURLResponse {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSCachedURLResponse {}

extern_methods!(
    unsafe impl NSCachedURLResponse {
        #[cfg(all(feature = "NSData", feature = "NSURLResponse"))]
        #[method_id(@__retain_semantics Init initWithResponse:data:)]
        pub unsafe fn initWithResponse_data(
            this: Allocated<Self>,
            response: &NSURLResponse,
            data: &NSData,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "NSData",
            feature = "NSDictionary",
            feature = "NSURLResponse"
        ))]
        #[method_id(@__retain_semantics Init initWithResponse:data:userInfo:storagePolicy:)]
        pub unsafe fn initWithResponse_data_userInfo_storagePolicy(
            this: Allocated<Self>,
            response: &NSURLResponse,
            data: &NSData,
            user_info: Option<&NSDictionary>,
            storage_policy: NSURLCacheStoragePolicy,
        ) -> Retained<Self>;

        #[cfg(feature = "NSURLResponse")]
        #[method_id(@__retain_semantics Other response)]
        pub unsafe fn response(&self) -> Retained<NSURLResponse>;

        #[cfg(feature = "NSData")]
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data(&self) -> Retained<NSData>;

        #[cfg(feature = "NSDictionary")]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Retained<NSDictionary>>;

        #[method(storagePolicy)]
        pub unsafe fn storagePolicy(&self) -> NSURLCacheStoragePolicy;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSCachedURLResponse {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSURLCache;

    unsafe impl ClassType for NSURLCache {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for NSURLCache {}

unsafe impl Sync for NSURLCache {}

unsafe impl NSObjectProtocol for NSURLCache {}

extern_methods!(
    unsafe impl NSURLCache {
        #[method_id(@__retain_semantics Other sharedURLCache)]
        pub unsafe fn sharedURLCache() -> Retained<NSURLCache>;

        #[method(setSharedURLCache:)]
        pub unsafe fn setSharedURLCache(shared_url_cache: &NSURLCache);

        #[cfg(feature = "NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithMemoryCapacity:diskCapacity:diskPath:)]
        pub unsafe fn initWithMemoryCapacity_diskCapacity_diskPath(
            this: Allocated<Self>,
            memory_capacity: NSUInteger,
            disk_capacity: NSUInteger,
            path: Option<&NSString>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Init initWithMemoryCapacity:diskCapacity:directoryURL:)]
        pub unsafe fn initWithMemoryCapacity_diskCapacity_directoryURL(
            this: Allocated<Self>,
            memory_capacity: NSUInteger,
            disk_capacity: NSUInteger,
            directory_url: Option<&NSURL>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSURLRequest")]
        #[method_id(@__retain_semantics Other cachedResponseForRequest:)]
        pub unsafe fn cachedResponseForRequest(
            &self,
            request: &NSURLRequest,
        ) -> Option<Retained<NSCachedURLResponse>>;

        #[cfg(feature = "NSURLRequest")]
        #[method(storeCachedResponse:forRequest:)]
        pub unsafe fn storeCachedResponse_forRequest(
            &self,
            cached_response: &NSCachedURLResponse,
            request: &NSURLRequest,
        );

        #[cfg(feature = "NSURLRequest")]
        #[method(removeCachedResponseForRequest:)]
        pub unsafe fn removeCachedResponseForRequest(&self, request: &NSURLRequest);

        #[method(removeAllCachedResponses)]
        pub unsafe fn removeAllCachedResponses(&self);

        #[cfg(feature = "NSDate")]
        #[method(removeCachedResponsesSinceDate:)]
        pub unsafe fn removeCachedResponsesSinceDate(&self, date: &NSDate);

        #[method(memoryCapacity)]
        pub unsafe fn memoryCapacity(&self) -> NSUInteger;

        #[method(setMemoryCapacity:)]
        pub unsafe fn setMemoryCapacity(&self, memory_capacity: NSUInteger);

        #[method(diskCapacity)]
        pub unsafe fn diskCapacity(&self) -> NSUInteger;

        #[method(setDiskCapacity:)]
        pub unsafe fn setDiskCapacity(&self, disk_capacity: NSUInteger);

        #[method(currentMemoryUsage)]
        pub unsafe fn currentMemoryUsage(&self) -> NSUInteger;

        #[method(currentDiskUsage)]
        pub unsafe fn currentDiskUsage(&self) -> NSUInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSURLCache {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSURLSessionTaskAdditions
    unsafe impl NSURLCache {
        #[cfg(feature = "NSURLSession")]
        #[method(storeCachedResponse:forDataTask:)]
        pub unsafe fn storeCachedResponse_forDataTask(
            &self,
            cached_response: &NSCachedURLResponse,
            data_task: &NSURLSessionDataTask,
        );

        #[cfg(all(feature = "NSURLSession", feature = "block2"))]
        #[method(getCachedResponseForDataTask:completionHandler:)]
        pub unsafe fn getCachedResponseForDataTask_completionHandler(
            &self,
            data_task: &NSURLSessionDataTask,
            completion_handler: &block2::Block<dyn Fn(*mut NSCachedURLResponse)>,
        );

        #[cfg(feature = "NSURLSession")]
        #[method(removeCachedResponseForDataTask:)]
        pub unsafe fn removeCachedResponseForDataTask(&self, data_task: &NSURLSessionDataTask);
    }
);
