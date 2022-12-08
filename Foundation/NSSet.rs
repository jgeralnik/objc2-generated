//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSSet<ObjectType: Message = Object, ObjectTypeOwnership: Ownership = Shared> {
        _inner0: PhantomData<*mut (ObjectType, ObjectTypeOwnership)>,
    }

    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership> ClassType
        for NSSet<ObjectType, ObjectTypeOwnership>
    {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSSet<ObjectType, ObjectTypeOwnership>
    {
        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other member:)]
        pub unsafe fn member(&self, object: &ObjectType) -> Option<Id<ObjectType, Shared>>;

        #[method_id(@__retain_semantics Other objectEnumerator)]
        pub unsafe fn objectEnumerator(&self) -> Id<NSEnumerator<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithObjects:count:)]
        pub unsafe fn initWithObjects_count(
            this: Option<Allocated<Self>>,
            objects: *mut NonNull<ObjectType>,
            cnt: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_methods!(
    /// NSExtendedSet
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSSet<ObjectType, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Other allObjects)]
        pub unsafe fn allObjects(&self) -> Id<NSArray<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other anyObject)]
        pub unsafe fn anyObject(&self) -> Option<Id<ObjectType, Shared>>;

        #[method(containsObject:)]
        pub unsafe fn containsObject(&self, anObject: &ObjectType) -> bool;

        #[method_id(@__retain_semantics Other description)]
        pub unsafe fn description(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other descriptionWithLocale:)]
        pub unsafe fn descriptionWithLocale(&self, locale: Option<&Object>)
            -> Id<NSString, Shared>;

        #[method(intersectsSet:)]
        pub unsafe fn intersectsSet(&self, otherSet: &NSSet<ObjectType>) -> bool;

        #[method(isEqualToSet:)]
        pub unsafe fn isEqualToSet(&self, otherSet: &NSSet<ObjectType>) -> bool;

        #[method(isSubsetOfSet:)]
        pub unsafe fn isSubsetOfSet(&self, otherSet: &NSSet<ObjectType>) -> bool;

        #[method(makeObjectsPerformSelector:)]
        pub unsafe fn makeObjectsPerformSelector(&self, aSelector: Sel);

        #[method(makeObjectsPerformSelector:withObject:)]
        pub unsafe fn makeObjectsPerformSelector_withObject(
            &self,
            aSelector: Sel,
            argument: Option<&Object>,
        );

        #[method_id(@__retain_semantics Other setByAddingObject:)]
        pub unsafe fn setByAddingObject(
            &self,
            anObject: &ObjectType,
        ) -> Id<NSSet<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other setByAddingObjectsFromSet:)]
        pub unsafe fn setByAddingObjectsFromSet(
            &self,
            other: &NSSet<ObjectType>,
        ) -> Id<NSSet<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other setByAddingObjectsFromArray:)]
        pub unsafe fn setByAddingObjectsFromArray(
            &self,
            other: &NSArray<ObjectType>,
        ) -> Id<NSSet<ObjectType>, Shared>;

        #[method(enumerateObjectsUsingBlock:)]
        pub unsafe fn enumerateObjectsUsingBlock(
            &self,
            block: &Block<(NonNull<ObjectType>, NonNull<Bool>), ()>,
        );

        #[method(enumerateObjectsWithOptions:usingBlock:)]
        pub unsafe fn enumerateObjectsWithOptions_usingBlock(
            &self,
            opts: NSEnumerationOptions,
            block: &Block<(NonNull<ObjectType>, NonNull<Bool>), ()>,
        );

        #[method_id(@__retain_semantics Other objectsPassingTest:)]
        pub unsafe fn objectsPassingTest(
            &self,
            predicate: &Block<(NonNull<ObjectType>, NonNull<Bool>), Bool>,
        ) -> Id<NSSet<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other objectsWithOptions:passingTest:)]
        pub unsafe fn objectsWithOptions_passingTest(
            &self,
            opts: NSEnumerationOptions,
            predicate: &Block<(NonNull<ObjectType>, NonNull<Bool>), Bool>,
        ) -> Id<NSSet<ObjectType>, Shared>;
    }
);

extern_methods!(
    /// NSSetCreation
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSSet<ObjectType, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Other set)]
        pub unsafe fn set() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other setWithObject:)]
        pub unsafe fn setWithObject(object: &ObjectType) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other setWithObjects:count:)]
        pub unsafe fn setWithObjects_count(
            objects: NonNull<NonNull<ObjectType>>,
            cnt: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other setWithSet:)]
        pub unsafe fn setWithSet(set: &NSSet<ObjectType>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other setWithArray:)]
        pub unsafe fn setWithArray(array: &NSArray<ObjectType>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithSet:)]
        pub unsafe fn initWithSet(
            this: Option<Allocated<Self>>,
            set: &NSSet<ObjectType>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithSet:copyItems:)]
        pub unsafe fn initWithSet_copyItems(
            this: Option<Allocated<Self>>,
            set: &NSSet<ObjectType>,
            flag: bool,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithArray:)]
        pub unsafe fn initWithArray(
            this: Option<Allocated<Self>>,
            array: &NSArray<ObjectType>,
        ) -> Id<Self, Shared>;
    }
);

__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSMutableSet<ObjectType: Message = Object, ObjectTypeOwnership: Ownership = Shared> {
        _inner0: PhantomData<*mut (ObjectType, ObjectTypeOwnership)>,
    }

    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership> ClassType
        for NSMutableSet<ObjectType, ObjectTypeOwnership>
    {
        type Super = NSSet;
    }
);

extern_methods!(
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSMutableSet<ObjectType, ObjectTypeOwnership>
    {
        #[method(addObject:)]
        pub unsafe fn addObject(&self, object: &ObjectType);

        #[method(removeObject:)]
        pub unsafe fn removeObject(&self, object: &ObjectType);

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCapacity:)]
        pub unsafe fn initWithCapacity(
            this: Option<Allocated<Self>>,
            numItems: NSUInteger,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// NSExtendedMutableSet
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSMutableSet<ObjectType, ObjectTypeOwnership>
    {
        #[method(addObjectsFromArray:)]
        pub unsafe fn addObjectsFromArray(&self, array: &NSArray<ObjectType>);

        #[method(intersectSet:)]
        pub unsafe fn intersectSet(&self, otherSet: &NSSet<ObjectType>);

        #[method(minusSet:)]
        pub unsafe fn minusSet(&self, otherSet: &NSSet<ObjectType>);

        #[method(removeAllObjects)]
        pub unsafe fn removeAllObjects(&self);

        #[method(unionSet:)]
        pub unsafe fn unionSet(&self, otherSet: &NSSet<ObjectType>);

        #[method(setSet:)]
        pub unsafe fn setSet(&self, otherSet: &NSSet<ObjectType>);
    }
);

extern_methods!(
    /// NSMutableSetCreation
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSMutableSet<ObjectType, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Other setWithCapacity:)]
        pub unsafe fn setWithCapacity(numItems: NSUInteger) -> Id<Self, Shared>;
    }
);

__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSCountedSet<ObjectType: Message = Object, ObjectTypeOwnership: Ownership = Shared> {
        _inner0: PhantomData<*mut (ObjectType, ObjectTypeOwnership)>,
    }

    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership> ClassType
        for NSCountedSet<ObjectType, ObjectTypeOwnership>
    {
        type Super = NSMutableSet;
    }
);

extern_methods!(
    unsafe impl<ObjectType: Message, ObjectTypeOwnership: Ownership>
        NSCountedSet<ObjectType, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Init initWithCapacity:)]
        pub unsafe fn initWithCapacity(
            this: Option<Allocated<Self>>,
            numItems: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithArray:)]
        pub unsafe fn initWithArray(
            this: Option<Allocated<Self>>,
            array: &NSArray<ObjectType>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithSet:)]
        pub unsafe fn initWithSet(
            this: Option<Allocated<Self>>,
            set: &NSSet<ObjectType>,
        ) -> Id<Self, Shared>;

        #[method(countForObject:)]
        pub unsafe fn countForObject(&self, object: &ObjectType) -> NSUInteger;

        #[method_id(@__retain_semantics Other objectEnumerator)]
        pub unsafe fn objectEnumerator(&self) -> Id<NSEnumerator<ObjectType>, Shared>;

        #[method(addObject:)]
        pub unsafe fn addObject(&self, object: &ObjectType);

        #[method(removeObject:)]
        pub unsafe fn removeObject(&self, object: &ObjectType);
    }
);
