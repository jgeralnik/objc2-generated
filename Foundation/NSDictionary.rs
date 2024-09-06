//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

__inner_extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub struct NSDictionary<KeyType: ?Sized = AnyObject, ObjectType: ?Sized = AnyObject> {
        __superclass: NSObject,
        _inner0: PhantomData<*mut KeyType>,
        _inner1: PhantomData<*mut ObjectType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<KeyType: ?Sized + Message, ObjectType: ?Sized + Message> ClassType
        for NSDictionary<KeyType, ObjectType>
    {
        type Super = NSObject;
        type Mutability = InteriorMutableWithSubclass<NSMutableDictionary<KeyType, ObjectType>>;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }
    }
);

#[cfg(feature = "NSObject")]
unsafe impl<KeyType: ?Sized + NSCoding, ObjectType: ?Sized + NSCoding> NSCoding
    for NSDictionary<KeyType, ObjectType>
{
}

#[cfg(feature = "NSObject")]
unsafe impl<KeyType: ?Sized, ObjectType: ?Sized> NSCopying for NSDictionary<KeyType, ObjectType> {}

#[cfg(feature = "NSEnumerator")]
unsafe impl<KeyType: ?Sized, ObjectType: ?Sized> NSFastEnumeration
    for NSDictionary<KeyType, ObjectType>
{
}

#[cfg(feature = "NSObject")]
unsafe impl<KeyType: ?Sized, ObjectType: ?Sized> NSMutableCopying
    for NSDictionary<KeyType, ObjectType>
{
}

unsafe impl<KeyType: ?Sized, ObjectType: ?Sized> NSObjectProtocol
    for NSDictionary<KeyType, ObjectType>
{
}

#[cfg(feature = "NSObject")]
unsafe impl<KeyType: ?Sized + NSSecureCoding, ObjectType: ?Sized + NSSecureCoding> NSSecureCoding
    for NSDictionary<KeyType, ObjectType>
{
}

extern_methods!(
    unsafe impl<KeyType: Message, ObjectType: Message> NSDictionary<KeyType, ObjectType> {
        #[method(count)]
        pub fn count(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other objectForKey:)]
        pub fn objectForKey(&self, a_key: &KeyType) -> Option<Retained<ObjectType>>;

        #[cfg(feature = "NSEnumerator")]
        #[method_id(@__retain_semantics Other keyEnumerator)]
        pub unsafe fn keyEnumerator(&self) -> Retained<NSEnumerator<KeyType>>;

        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "NSObject")]
        #[method_id(@__retain_semantics Init initWithObjects:forKeys:count:)]
        pub unsafe fn initWithObjects_forKeys_count(
            this: Allocated<Self>,
            objects: *mut NonNull<ObjectType>,
            keys: *mut NonNull<ProtocolObject<dyn NSCopying>>,
            cnt: NSUInteger,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<KeyType: Message, ObjectType: Message> NSDictionary<KeyType, ObjectType> {
        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl<KeyType: Message, ObjectType: Message> DefaultRetained for NSDictionary<KeyType, ObjectType> {
    #[inline]
    fn default_id() -> Retained<Self> {
        Self::new()
    }
}

extern_methods!(
    /// NSExtendedDictionary
    unsafe impl<KeyType: Message, ObjectType: Message> NSDictionary<KeyType, ObjectType> {
        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other allKeys)]
        pub fn allKeys(&self) -> Retained<NSArray<KeyType>>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other allKeysForObject:)]
        pub unsafe fn allKeysForObject(&self, an_object: &ObjectType)
            -> Retained<NSArray<KeyType>>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other allValues)]
        pub fn allValues(&self) -> Retained<NSArray<ObjectType>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other description)]
        pub unsafe fn description(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other descriptionInStringsFileFormat)]
        pub unsafe fn descriptionInStringsFileFormat(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other descriptionWithLocale:)]
        pub unsafe fn descriptionWithLocale(
            &self,
            locale: Option<&AnyObject>,
        ) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other descriptionWithLocale:indent:)]
        pub unsafe fn descriptionWithLocale_indent(
            &self,
            locale: Option<&AnyObject>,
            level: NSUInteger,
        ) -> Retained<NSString>;

        #[method(isEqualToDictionary:)]
        pub unsafe fn isEqualToDictionary(
            &self,
            other_dictionary: &NSDictionary<KeyType, ObjectType>,
        ) -> bool;

        #[cfg(feature = "NSEnumerator")]
        #[method_id(@__retain_semantics Other objectEnumerator)]
        pub unsafe fn objectEnumerator(&self) -> Retained<NSEnumerator<ObjectType>>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other objectsForKeys:notFoundMarker:)]
        pub unsafe fn objectsForKeys_notFoundMarker(
            &self,
            keys: &NSArray<KeyType>,
            marker: &ObjectType,
        ) -> Retained<NSArray<ObjectType>>;

        #[cfg(all(feature = "NSError", feature = "NSURL"))]
        #[method(writeToURL:error:_)]
        pub unsafe fn writeToURL_error(&self, url: &NSURL) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other keysSortedByValueUsingSelector:)]
        pub unsafe fn keysSortedByValueUsingSelector(
            &self,
            comparator: Sel,
        ) -> Retained<NSArray<KeyType>>;

        #[method(getObjects:andKeys:count:)]
        pub unsafe fn getObjects_andKeys_count(
            &self,
            objects: *mut NonNull<ObjectType>,
            keys: *mut NonNull<KeyType>,
            count: NSUInteger,
        );

        #[method_id(@__retain_semantics Other objectForKeyedSubscript:)]
        pub unsafe fn objectForKeyedSubscript(&self, key: &KeyType)
            -> Option<Retained<ObjectType>>;

        #[cfg(feature = "block2")]
        #[method(enumerateKeysAndObjectsUsingBlock:)]
        pub unsafe fn enumerateKeysAndObjectsUsingBlock(
            &self,
            block: &block2::Block<
                dyn Fn(NonNull<KeyType>, NonNull<ObjectType>, NonNull<Bool>) + '_,
            >,
        );

        #[cfg(all(feature = "NSObjCRuntime", feature = "block2"))]
        #[method(enumerateKeysAndObjectsWithOptions:usingBlock:)]
        pub unsafe fn enumerateKeysAndObjectsWithOptions_usingBlock(
            &self,
            opts: NSEnumerationOptions,
            block: &block2::Block<
                dyn Fn(NonNull<KeyType>, NonNull<ObjectType>, NonNull<Bool>) + '_,
            >,
        );

        #[cfg(all(feature = "NSArray", feature = "NSObjCRuntime", feature = "block2"))]
        #[method_id(@__retain_semantics Other keysSortedByValueUsingComparator:)]
        pub unsafe fn keysSortedByValueUsingComparator(
            &self,
            cmptr: NSComparator,
        ) -> Retained<NSArray<KeyType>>;

        #[cfg(all(feature = "NSArray", feature = "NSObjCRuntime", feature = "block2"))]
        #[method_id(@__retain_semantics Other keysSortedByValueWithOptions:usingComparator:)]
        pub unsafe fn keysSortedByValueWithOptions_usingComparator(
            &self,
            opts: NSSortOptions,
            cmptr: NSComparator,
        ) -> Retained<NSArray<KeyType>>;

        #[cfg(all(feature = "NSSet", feature = "block2"))]
        #[method_id(@__retain_semantics Other keysOfEntriesPassingTest:)]
        pub unsafe fn keysOfEntriesPassingTest(
            &self,
            predicate: &block2::Block<
                dyn Fn(NonNull<KeyType>, NonNull<ObjectType>, NonNull<Bool>) -> Bool + '_,
            >,
        ) -> Retained<NSSet<KeyType>>;

        #[cfg(all(feature = "NSObjCRuntime", feature = "NSSet", feature = "block2"))]
        #[method_id(@__retain_semantics Other keysOfEntriesWithOptions:passingTest:)]
        pub unsafe fn keysOfEntriesWithOptions_passingTest(
            &self,
            opts: NSEnumerationOptions,
            predicate: &block2::Block<
                dyn Fn(NonNull<KeyType>, NonNull<ObjectType>, NonNull<Bool>) -> Bool + '_,
            >,
        ) -> Retained<NSSet<KeyType>>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl<KeyType: Message, ObjectType: Message> NSDictionary<KeyType, ObjectType> {
        #[deprecated = "Use -getObjects:andKeys:count: instead"]
        #[method(getObjects:andKeys:)]
        pub unsafe fn getObjects_andKeys(
            &self,
            objects: *mut NonNull<ObjectType>,
            keys: *mut NonNull<KeyType>,
        );

        #[cfg(feature = "NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other dictionaryWithContentsOfFile:)]
        pub unsafe fn dictionaryWithContentsOfFile(
            path: &NSString,
        ) -> Option<Retained<NSDictionary<KeyType, ObjectType>>>;

        #[cfg(feature = "NSURL")]
        #[deprecated]
        #[method_id(@__retain_semantics Other dictionaryWithContentsOfURL:)]
        pub unsafe fn dictionaryWithContentsOfURL(
            url: &NSURL,
        ) -> Option<Retained<NSDictionary<KeyType, ObjectType>>>;

        #[cfg(feature = "NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Allocated<Self>,
            path: &NSString,
        ) -> Option<Retained<NSDictionary<KeyType, ObjectType>>>;

        #[cfg(feature = "NSURL")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Allocated<Self>,
            url: &NSURL,
        ) -> Option<Retained<NSDictionary<KeyType, ObjectType>>>;

        #[cfg(feature = "NSString")]
        #[deprecated]
        #[method(writeToFile:atomically:)]
        pub unsafe fn writeToFile_atomically(
            &self,
            path: &NSString,
            use_auxiliary_file: bool,
        ) -> bool;

        #[cfg(feature = "NSURL")]
        #[deprecated]
        #[method(writeToURL:atomically:)]
        pub unsafe fn writeToURL_atomically(&self, url: &NSURL, atomically: bool) -> bool;
    }
);

extern_methods!(
    /// NSDictionaryCreation
    unsafe impl<KeyType: Message, ObjectType: Message> NSDictionary<KeyType, ObjectType> {
        #[method_id(@__retain_semantics Other dictionary)]
        pub unsafe fn dictionary() -> Retained<Self>;

        #[cfg(feature = "NSObject")]
        #[method_id(@__retain_semantics Other dictionaryWithObject:forKey:)]
        pub unsafe fn dictionaryWithObject_forKey(
            object: &ObjectType,
            key: &ProtocolObject<dyn NSCopying>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSObject")]
        #[method_id(@__retain_semantics Other dictionaryWithObjects:forKeys:count:)]
        pub unsafe fn dictionaryWithObjects_forKeys_count(
            objects: *mut NonNull<ObjectType>,
            keys: *mut NonNull<ProtocolObject<dyn NSCopying>>,
            cnt: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other dictionaryWithDictionary:)]
        pub unsafe fn dictionaryWithDictionary(
            dict: &NSDictionary<KeyType, ObjectType>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSArray", feature = "NSObject"))]
        #[method_id(@__retain_semantics Other dictionaryWithObjects:forKeys:)]
        pub unsafe fn dictionaryWithObjects_forKeys(
            objects: &NSArray<ObjectType>,
            keys: &NSArray<ProtocolObject<dyn NSCopying>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDictionary:)]
        pub unsafe fn initWithDictionary(
            this: Allocated<Self>,
            other_dictionary: &NSDictionary<KeyType, ObjectType>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDictionary:copyItems:)]
        pub unsafe fn initWithDictionary_copyItems(
            this: Allocated<Self>,
            other_dictionary: &NSDictionary<KeyType, ObjectType>,
            flag: bool,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSArray", feature = "NSObject"))]
        #[method_id(@__retain_semantics Init initWithObjects:forKeys:)]
        pub unsafe fn initWithObjects_forKeys(
            this: Allocated<Self>,
            objects: &NSArray<ObjectType>,
            keys: &NSArray<ProtocolObject<dyn NSCopying>>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSDictionary`
    ///
    /// NSDictionaryCreation
    unsafe impl<KeyType: Message, ObjectType: Message> NSMutableDictionary<KeyType, ObjectType> {
        #[method_id(@__retain_semantics Other dictionary)]
        pub unsafe fn dictionary() -> Retained<Self>;

        #[cfg(feature = "NSObject")]
        #[method_id(@__retain_semantics Other dictionaryWithObject:forKey:)]
        pub unsafe fn dictionaryWithObject_forKey(
            object: &ObjectType,
            key: &ProtocolObject<dyn NSCopying>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSObject")]
        #[method_id(@__retain_semantics Other dictionaryWithObjects:forKeys:count:)]
        pub unsafe fn dictionaryWithObjects_forKeys_count(
            objects: *mut NonNull<ObjectType>,
            keys: *mut NonNull<ProtocolObject<dyn NSCopying>>,
            cnt: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other dictionaryWithDictionary:)]
        pub unsafe fn dictionaryWithDictionary(
            dict: &NSDictionary<KeyType, ObjectType>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSArray", feature = "NSObject"))]
        #[method_id(@__retain_semantics Other dictionaryWithObjects:forKeys:)]
        pub unsafe fn dictionaryWithObjects_forKeys(
            objects: &NSArray<ObjectType>,
            keys: &NSArray<ProtocolObject<dyn NSCopying>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDictionary:)]
        pub unsafe fn initWithDictionary(
            this: Allocated<Self>,
            other_dictionary: &NSDictionary<KeyType, ObjectType>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDictionary:copyItems:)]
        pub unsafe fn initWithDictionary_copyItems(
            this: Allocated<Self>,
            other_dictionary: &NSDictionary<KeyType, ObjectType>,
            flag: bool,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSArray", feature = "NSObject"))]
        #[method_id(@__retain_semantics Init initWithObjects:forKeys:)]
        pub unsafe fn initWithObjects_forKeys(
            this: Allocated<Self>,
            objects: &NSArray<ObjectType>,
            keys: &NSArray<ProtocolObject<dyn NSCopying>>,
        ) -> Retained<Self>;
    }
);

__inner_extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub struct NSMutableDictionary<KeyType: ?Sized = AnyObject, ObjectType: ?Sized = AnyObject> {
        __superclass: NSDictionary<KeyType, ObjectType>,
        _inner0: PhantomData<*mut KeyType>,
        _inner1: PhantomData<*mut ObjectType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<KeyType: ?Sized + Message, ObjectType: ?Sized + Message> ClassType
        for NSMutableDictionary<KeyType, ObjectType>
    {
        #[inherits(NSObject)]
        type Super = NSDictionary<KeyType, ObjectType>;
        type Mutability = InteriorMutableWithSuperclass<NSDictionary<KeyType, ObjectType>>;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }
    }
);

#[cfg(feature = "NSObject")]
unsafe impl<KeyType: ?Sized + NSCoding, ObjectType: ?Sized + NSCoding> NSCoding
    for NSMutableDictionary<KeyType, ObjectType>
{
}

#[cfg(feature = "NSObject")]
unsafe impl<KeyType: ?Sized, ObjectType: ?Sized> NSCopying
    for NSMutableDictionary<KeyType, ObjectType>
{
}

#[cfg(feature = "NSEnumerator")]
unsafe impl<KeyType: ?Sized, ObjectType: ?Sized> NSFastEnumeration
    for NSMutableDictionary<KeyType, ObjectType>
{
}

#[cfg(feature = "NSObject")]
unsafe impl<KeyType: ?Sized, ObjectType: ?Sized> NSMutableCopying
    for NSMutableDictionary<KeyType, ObjectType>
{
}

unsafe impl<KeyType: ?Sized, ObjectType: ?Sized> NSObjectProtocol
    for NSMutableDictionary<KeyType, ObjectType>
{
}

#[cfg(feature = "NSObject")]
unsafe impl<KeyType: ?Sized + NSSecureCoding, ObjectType: ?Sized + NSSecureCoding> NSSecureCoding
    for NSMutableDictionary<KeyType, ObjectType>
{
}

extern_methods!(
    unsafe impl<KeyType: Message, ObjectType: Message> NSMutableDictionary<KeyType, ObjectType> {
        #[method(removeObjectForKey:)]
        pub fn removeObjectForKey(&self, a_key: &KeyType);

        #[cfg(feature = "NSObject")]
        #[method(setObject:forKey:)]
        pub unsafe fn setObject_forKey(
            &self,
            an_object: &ObjectType,
            a_key: &ProtocolObject<dyn NSCopying>,
        );

        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCapacity:)]
        pub fn initWithCapacity(this: Allocated<Self>, num_items: NSUInteger) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSDictionary`
    unsafe impl<KeyType: Message, ObjectType: Message> NSMutableDictionary<KeyType, ObjectType> {
        #[cfg(feature = "NSObject")]
        #[method_id(@__retain_semantics Init initWithObjects:forKeys:count:)]
        pub unsafe fn initWithObjects_forKeys_count(
            this: Allocated<Self>,
            objects: *mut NonNull<ObjectType>,
            keys: *mut NonNull<ProtocolObject<dyn NSCopying>>,
            cnt: NSUInteger,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<KeyType: Message, ObjectType: Message> NSMutableDictionary<KeyType, ObjectType> {
        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl<KeyType: Message, ObjectType: Message> DefaultRetained
    for NSMutableDictionary<KeyType, ObjectType>
{
    #[inline]
    fn default_id() -> Retained<Self> {
        Self::new()
    }
}

extern_methods!(
    /// NSExtendedMutableDictionary
    unsafe impl<KeyType: Message, ObjectType: Message> NSMutableDictionary<KeyType, ObjectType> {
        #[method(addEntriesFromDictionary:)]
        pub unsafe fn addEntriesFromDictionary(
            &self,
            other_dictionary: &NSDictionary<KeyType, ObjectType>,
        );

        #[method(removeAllObjects)]
        pub fn removeAllObjects(&self);

        #[cfg(feature = "NSArray")]
        #[method(removeObjectsForKeys:)]
        pub unsafe fn removeObjectsForKeys(&self, key_array: &NSArray<KeyType>);

        #[method(setDictionary:)]
        pub unsafe fn setDictionary(&self, other_dictionary: &NSDictionary<KeyType, ObjectType>);

        #[cfg(feature = "NSObject")]
        #[method(setObject:forKeyedSubscript:)]
        pub unsafe fn setObject_forKeyedSubscript(
            &self,
            obj: Option<&ObjectType>,
            key: &ProtocolObject<dyn NSCopying>,
        );
    }
);

extern_methods!(
    /// NSMutableDictionaryCreation
    unsafe impl<KeyType: Message, ObjectType: Message> NSMutableDictionary<KeyType, ObjectType> {
        #[method_id(@__retain_semantics Other dictionaryWithCapacity:)]
        pub unsafe fn dictionaryWithCapacity(num_items: NSUInteger) -> Retained<Self>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other dictionaryWithContentsOfFile:)]
        pub unsafe fn dictionaryWithContentsOfFile(
            path: &NSString,
        ) -> Option<Retained<NSMutableDictionary<KeyType, ObjectType>>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other dictionaryWithContentsOfURL:)]
        pub unsafe fn dictionaryWithContentsOfURL(
            url: &NSURL,
        ) -> Option<Retained<NSMutableDictionary<KeyType, ObjectType>>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Allocated<Self>,
            path: &NSString,
        ) -> Option<Retained<NSMutableDictionary<KeyType, ObjectType>>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Allocated<Self>,
            url: &NSURL,
        ) -> Option<Retained<NSMutableDictionary<KeyType, ObjectType>>>;
    }
);

extern_methods!(
    /// NSSharedKeySetDictionary
    unsafe impl<KeyType: Message, ObjectType: Message> NSDictionary<KeyType, ObjectType> {
        #[cfg(all(feature = "NSArray", feature = "NSObject"))]
        #[method_id(@__retain_semantics Other sharedKeySetForKeys:)]
        pub unsafe fn sharedKeySetForKeys(
            keys: &NSArray<ProtocolObject<dyn NSCopying>>,
        ) -> Retained<AnyObject>;
    }
);

extern_methods!(
    /// NSSharedKeySetDictionary
    unsafe impl<KeyType: Message, ObjectType: Message> NSMutableDictionary<KeyType, ObjectType> {
        #[method_id(@__retain_semantics Other dictionaryWithSharedKeySet:)]
        pub unsafe fn dictionaryWithSharedKeySet(
            keyset: &AnyObject,
        ) -> Retained<NSMutableDictionary<KeyType, ObjectType>>;
    }
);
