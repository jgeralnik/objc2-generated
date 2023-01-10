//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSDecodingFailurePolicy {
        NSDecodingFailurePolicyRaiseException = 0,
        NSDecodingFailurePolicySetErrorAndReturn = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCoder;

    unsafe impl ClassType for NSCoder {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSCoder")]
    unsafe impl NSCoder {
        #[method(encodeValueOfObjCType:at:)]
        pub unsafe fn encodeValueOfObjCType_at(
            &self,
            type_: NonNull<c_char>,
            addr: NonNull<c_void>,
        );

        #[method(encodeDataObject:)]
        pub unsafe fn encodeDataObject(&self, data: &NSData);

        #[method_id(@__retain_semantics Other decodeDataObject)]
        pub unsafe fn decodeDataObject(&self) -> Option<Id<NSData, Shared>>;

        #[method(decodeValueOfObjCType:at:size:)]
        pub unsafe fn decodeValueOfObjCType_at_size(
            &self,
            type_: NonNull<c_char>,
            data: NonNull<c_void>,
            size: NSUInteger,
        );

        #[method(versionForClassName:)]
        pub unsafe fn versionForClassName(&self, className: &NSString) -> NSInteger;
    }
);

extern_methods!(
    /// NSExtendedCoder
    #[cfg(feature = "Foundation_NSCoder")]
    unsafe impl NSCoder {
        #[method(encodeObject:)]
        pub unsafe fn encodeObject(&self, object: Option<&Object>);

        #[method(encodeRootObject:)]
        pub unsafe fn encodeRootObject(&self, rootObject: &Object);

        #[method(encodeBycopyObject:)]
        pub unsafe fn encodeBycopyObject(&self, anObject: Option<&Object>);

        #[method(encodeByrefObject:)]
        pub unsafe fn encodeByrefObject(&self, anObject: Option<&Object>);

        #[method(encodeConditionalObject:)]
        pub unsafe fn encodeConditionalObject(&self, object: Option<&Object>);

        #[method(encodeArrayOfObjCType:count:at:)]
        pub unsafe fn encodeArrayOfObjCType_count_at(
            &self,
            type_: NonNull<c_char>,
            count: NSUInteger,
            array: NonNull<c_void>,
        );

        #[method(encodeBytes:length:)]
        pub unsafe fn encodeBytes_length(&self, byteaddr: *mut c_void, length: NSUInteger);

        #[method_id(@__retain_semantics Other decodeObject)]
        pub unsafe fn decodeObject(&self) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other decodeTopLevelObjectAndReturnError:_)]
        pub unsafe fn decodeTopLevelObjectAndReturnError(
            &self,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;

        #[method(decodeArrayOfObjCType:count:at:)]
        pub unsafe fn decodeArrayOfObjCType_count_at(
            &self,
            itemType: NonNull<c_char>,
            count: NSUInteger,
            array: NonNull<c_void>,
        );

        #[method(decodeBytesWithReturnedLength:)]
        pub unsafe fn decodeBytesWithReturnedLength(
            &self,
            lengthp: NonNull<NSUInteger>,
        ) -> *mut c_void;

        #[method(encodePropertyList:)]
        pub unsafe fn encodePropertyList(&self, aPropertyList: &Object);

        #[method_id(@__retain_semantics Other decodePropertyList)]
        pub unsafe fn decodePropertyList(&self) -> Option<Id<Object, Shared>>;

        #[method(setObjectZone:)]
        pub unsafe fn setObjectZone(&self, zone: *mut NSZone);

        #[method(objectZone)]
        pub unsafe fn objectZone(&self) -> *mut NSZone;

        #[method(systemVersion)]
        pub unsafe fn systemVersion(&self) -> c_uint;

        #[method(allowsKeyedCoding)]
        pub unsafe fn allowsKeyedCoding(&self) -> bool;

        #[method(encodeObject:forKey:)]
        pub unsafe fn encodeObject_forKey(&self, object: Option<&Object>, key: &NSString);

        #[method(encodeConditionalObject:forKey:)]
        pub unsafe fn encodeConditionalObject_forKey(
            &self,
            object: Option<&Object>,
            key: &NSString,
        );

        #[method(encodeBool:forKey:)]
        pub unsafe fn encodeBool_forKey(&self, value: bool, key: &NSString);

        #[method(encodeInt:forKey:)]
        pub unsafe fn encodeInt_forKey(&self, value: c_int, key: &NSString);

        #[method(encodeInt32:forKey:)]
        pub unsafe fn encodeInt32_forKey(&self, value: i32, key: &NSString);

        #[method(encodeInt64:forKey:)]
        pub unsafe fn encodeInt64_forKey(&self, value: i64, key: &NSString);

        #[method(encodeFloat:forKey:)]
        pub unsafe fn encodeFloat_forKey(&self, value: c_float, key: &NSString);

        #[method(encodeDouble:forKey:)]
        pub unsafe fn encodeDouble_forKey(&self, value: c_double, key: &NSString);

        #[method(encodeBytes:length:forKey:)]
        pub unsafe fn encodeBytes_length_forKey(
            &self,
            bytes: *mut u8,
            length: NSUInteger,
            key: &NSString,
        );

        #[method(containsValueForKey:)]
        pub unsafe fn containsValueForKey(&self, key: &NSString) -> bool;

        #[method_id(@__retain_semantics Other decodeObjectForKey:)]
        pub unsafe fn decodeObjectForKey(&self, key: &NSString) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other decodeTopLevelObjectForKey:error:_)]
        pub unsafe fn decodeTopLevelObjectForKey_error(
            &self,
            key: &NSString,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;

        #[method(decodeBoolForKey:)]
        pub unsafe fn decodeBoolForKey(&self, key: &NSString) -> bool;

        #[method(decodeIntForKey:)]
        pub unsafe fn decodeIntForKey(&self, key: &NSString) -> c_int;

        #[method(decodeInt32ForKey:)]
        pub unsafe fn decodeInt32ForKey(&self, key: &NSString) -> i32;

        #[method(decodeInt64ForKey:)]
        pub unsafe fn decodeInt64ForKey(&self, key: &NSString) -> i64;

        #[method(decodeFloatForKey:)]
        pub unsafe fn decodeFloatForKey(&self, key: &NSString) -> c_float;

        #[method(decodeDoubleForKey:)]
        pub unsafe fn decodeDoubleForKey(&self, key: &NSString) -> c_double;

        #[method(decodeBytesForKey:returnedLength:)]
        pub unsafe fn decodeBytesForKey_returnedLength(
            &self,
            key: &NSString,
            lengthp: *mut NSUInteger,
        ) -> *mut u8;

        #[method(encodeInteger:forKey:)]
        pub unsafe fn encodeInteger_forKey(&self, value: NSInteger, key: &NSString);

        #[method(decodeIntegerForKey:)]
        pub unsafe fn decodeIntegerForKey(&self, key: &NSString) -> NSInteger;

        #[method(requiresSecureCoding)]
        pub unsafe fn requiresSecureCoding(&self) -> bool;

        #[method_id(@__retain_semantics Other decodeObjectOfClass:forKey:)]
        pub unsafe fn decodeObjectOfClass_forKey(
            &self,
            aClass: &Class,
            key: &NSString,
        ) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other decodeTopLevelObjectOfClass:forKey:error:_)]
        pub unsafe fn decodeTopLevelObjectOfClass_forKey_error(
            &self,
            aClass: &Class,
            key: &NSString,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other decodeArrayOfObjectsOfClass:forKey:)]
        pub unsafe fn decodeArrayOfObjectsOfClass_forKey(
            &self,
            cls: &Class,
            key: &NSString,
        ) -> Option<Id<NSArray, Shared>>;

        #[method_id(@__retain_semantics Other decodeDictionaryWithKeysOfClass:objectsOfClass:forKey:)]
        pub unsafe fn decodeDictionaryWithKeysOfClass_objectsOfClass_forKey(
            &self,
            keyCls: &Class,
            objectCls: &Class,
            key: &NSString,
        ) -> Option<Id<NSDictionary, Shared>>;

        #[method_id(@__retain_semantics Other decodeObjectOfClasses:forKey:)]
        pub unsafe fn decodeObjectOfClasses_forKey(
            &self,
            classes: Option<&NSSet<TodoClass>>,
            key: &NSString,
        ) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other decodeTopLevelObjectOfClasses:forKey:error:_)]
        pub unsafe fn decodeTopLevelObjectOfClasses_forKey_error(
            &self,
            classes: Option<&NSSet<TodoClass>>,
            key: &NSString,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other decodeArrayOfObjectsOfClasses:forKey:)]
        pub unsafe fn decodeArrayOfObjectsOfClasses_forKey(
            &self,
            classes: &NSSet<TodoClass>,
            key: &NSString,
        ) -> Option<Id<NSArray, Shared>>;

        #[method_id(@__retain_semantics Other decodeDictionaryWithKeysOfClasses:objectsOfClasses:forKey:)]
        pub unsafe fn decodeDictionaryWithKeysOfClasses_objectsOfClasses_forKey(
            &self,
            keyClasses: &NSSet<TodoClass>,
            objectClasses: &NSSet<TodoClass>,
            key: &NSString,
        ) -> Option<Id<NSDictionary, Shared>>;

        #[method_id(@__retain_semantics Other decodePropertyListForKey:)]
        pub unsafe fn decodePropertyListForKey(&self, key: &NSString)
            -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other allowedClasses)]
        pub unsafe fn allowedClasses(&self) -> Option<Id<NSSet<TodoClass>, Shared>>;

        #[method(failWithError:)]
        pub unsafe fn failWithError(&self, error: &NSError);

        #[method(decodingFailurePolicy)]
        pub unsafe fn decodingFailurePolicy(&self) -> NSDecodingFailurePolicy;

        #[method_id(@__retain_semantics Other error)]
        pub unsafe fn error(&self) -> Option<Id<NSError, Shared>>;
    }
);

extern_fn!(
    pub unsafe fn NXReadNSObjectFromCoder(decoder: &NSCoder) -> *mut NSObject;
);

extern_methods!(
    /// NSTypedstreamCompatibility
    #[cfg(feature = "Foundation_NSCoder")]
    unsafe impl NSCoder {
        #[method(encodeNXObject:)]
        pub unsafe fn encodeNXObject(&self, object: &Object);

        #[method_id(@__retain_semantics Other decodeNXObject)]
        pub unsafe fn decodeNXObject(&self) -> Option<Id<Object, Shared>>;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "Foundation_NSCoder")]
    unsafe impl NSCoder {
        #[method(decodeValueOfObjCType:at:)]
        pub unsafe fn decodeValueOfObjCType_at(
            &self,
            type_: NonNull<c_char>,
            data: NonNull<c_void>,
        );
    }
);
