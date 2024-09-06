//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSCoder")]
    #[deprecated = "Use NSKeyedArchiver instead"]
    pub struct NSArchiver;

    #[cfg(feature = "NSCoder")]
    unsafe impl ClassType for NSArchiver {
        #[inherits(NSObject)]
        type Super = NSCoder;
    }
);

#[cfg(feature = "NSCoder")]
unsafe impl NSObjectProtocol for NSArchiver {}

extern_methods!(
    #[cfg(feature = "NSCoder")]
    unsafe impl NSArchiver {
        #[cfg(feature = "NSData")]
        #[deprecated = "Use NSKeyedArchiver instead"]
        #[method_id(@__retain_semantics Init initForWritingWithMutableData:)]
        pub unsafe fn initForWritingWithMutableData(
            this: Allocated<Self>,
            mdata: &NSMutableData,
        ) -> Retained<Self>;

        #[cfg(feature = "NSData")]
        #[deprecated = "Use NSKeyedArchiver instead"]
        #[method_id(@__retain_semantics Other archiverData)]
        pub unsafe fn archiverData(&self) -> Retained<NSMutableData>;

        #[deprecated = "Use NSKeyedArchiver instead"]
        #[method(encodeRootObject:)]
        pub unsafe fn encodeRootObject(&self, root_object: &AnyObject);

        #[deprecated = "Use NSKeyedArchiver instead"]
        #[method(encodeConditionalObject:)]
        pub unsafe fn encodeConditionalObject(&self, object: Option<&AnyObject>);

        #[cfg(feature = "NSData")]
        #[deprecated = "Use NSKeyedArchiver instead"]
        #[method_id(@__retain_semantics Other archivedDataWithRootObject:)]
        pub unsafe fn archivedDataWithRootObject(root_object: &AnyObject) -> Retained<NSData>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use NSKeyedArchiver instead"]
        #[method(archiveRootObject:toFile:)]
        pub unsafe fn archiveRootObject_toFile(root_object: &AnyObject, path: &NSString) -> bool;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use NSKeyedArchiver instead"]
        #[method(encodeClassName:intoClassName:)]
        pub unsafe fn encodeClassName_intoClassName(
            &self,
            true_name: &NSString,
            in_archive_name: &NSString,
        );

        #[cfg(feature = "NSString")]
        #[deprecated = "Use NSKeyedArchiver instead"]
        #[method_id(@__retain_semantics Other classNameEncodedForTrueClassName:)]
        pub unsafe fn classNameEncodedForTrueClassName(
            &self,
            true_name: &NSString,
        ) -> Option<Retained<NSString>>;

        #[deprecated = "Use NSKeyedArchiver instead"]
        #[method(replaceObject:withObject:)]
        pub unsafe fn replaceObject_withObject(&self, object: &AnyObject, new_object: &AnyObject);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSCoder")]
    unsafe impl NSArchiver {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSCoder")]
    #[deprecated = "Use NSKeyedUnarchiver instead"]
    pub struct NSUnarchiver;

    #[cfg(feature = "NSCoder")]
    unsafe impl ClassType for NSUnarchiver {
        #[inherits(NSObject)]
        type Super = NSCoder;
    }
);

#[cfg(feature = "NSCoder")]
unsafe impl NSObjectProtocol for NSUnarchiver {}

extern_methods!(
    #[cfg(feature = "NSCoder")]
    unsafe impl NSUnarchiver {
        #[cfg(feature = "NSData")]
        #[deprecated = "Use NSKeyedUnarchiver instead"]
        #[method_id(@__retain_semantics Init initForReadingWithData:)]
        pub unsafe fn initForReadingWithData(
            this: Allocated<Self>,
            data: &NSData,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSZone")]
        #[deprecated = "Use NSKeyedUnarchiver instead"]
        #[method(setObjectZone:)]
        pub unsafe fn setObjectZone(&self, zone: *mut NSZone);

        #[cfg(feature = "NSZone")]
        #[deprecated = "Use NSKeyedUnarchiver instead"]
        #[method(objectZone)]
        pub unsafe fn objectZone(&self) -> *mut NSZone;

        #[deprecated = "Use NSKeyedUnarchiver instead"]
        #[method(isAtEnd)]
        pub unsafe fn isAtEnd(&self) -> bool;

        #[deprecated = "Use NSKeyedUnarchiver instead"]
        #[method(systemVersion)]
        pub unsafe fn systemVersion(&self) -> c_uint;

        #[cfg(feature = "NSData")]
        #[deprecated = "Use NSKeyedUnarchiver instead"]
        #[method_id(@__retain_semantics Other unarchiveObjectWithData:)]
        pub unsafe fn unarchiveObjectWithData(data: &NSData) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use NSKeyedUnarchiver instead"]
        #[method_id(@__retain_semantics Other unarchiveObjectWithFile:)]
        pub unsafe fn unarchiveObjectWithFile(path: &NSString) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use NSKeyedUnarchiver instead"]
        #[method(decodeClassName:asClassName:)]
        pub unsafe fn decodeClassName_asClassName_class(
            in_archive_name: &NSString,
            true_name: &NSString,
        );

        #[cfg(feature = "NSString")]
        #[deprecated = "Use NSKeyedUnarchiver instead"]
        #[method(decodeClassName:asClassName:)]
        pub unsafe fn decodeClassName_asClassName(
            &self,
            in_archive_name: &NSString,
            true_name: &NSString,
        );

        #[cfg(feature = "NSString")]
        #[deprecated = "Use NSKeyedUnarchiver instead"]
        #[method_id(@__retain_semantics Other classNameDecodedForArchiveClassName:)]
        pub unsafe fn classNameDecodedForArchiveClassName_class(
            in_archive_name: &NSString,
        ) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use NSKeyedUnarchiver instead"]
        #[method_id(@__retain_semantics Other classNameDecodedForArchiveClassName:)]
        pub unsafe fn classNameDecodedForArchiveClassName(
            &self,
            in_archive_name: &NSString,
        ) -> Retained<NSString>;

        #[deprecated = "Use NSKeyedUnarchiver instead"]
        #[method(replaceObject:withObject:)]
        pub unsafe fn replaceObject_withObject(&self, object: &AnyObject, new_object: &AnyObject);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSCoder")]
    unsafe impl NSUnarchiver {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_category!(
    /// Category "NSArchiverCallback" on [`NSObject`].
    #[doc(alias = "NSArchiverCallback")]
    pub unsafe trait NSObjectNSArchiverCallback {
        #[method(classForArchiver)]
        unsafe fn classForArchiver(&self) -> Option<&'static AnyClass>;

        #[cfg(feature = "NSCoder")]
        #[deprecated]
        #[method_id(@__retain_semantics Other replacementObjectForArchiver:)]
        unsafe fn replacementObjectForArchiver(
            &self,
            archiver: &NSArchiver,
        ) -> Option<Retained<AnyObject>>;
    }

    unsafe impl NSObjectNSArchiverCallback for NSObject {}
);
