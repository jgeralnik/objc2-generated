//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTreeNode;

    unsafe impl ClassType for NSTreeNode {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTreeNode")]
    unsafe impl NSTreeNode {
        #[method_id(@__retain_semantics Other treeNodeWithRepresentedObject:)]
        pub unsafe fn treeNodeWithRepresentedObject(
            modelObject: Option<&Object>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithRepresentedObject:)]
        pub unsafe fn initWithRepresentedObject(
            this: Option<Allocated<Self>>,
            modelObject: Option<&Object>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other representedObject)]
        pub unsafe fn representedObject(&self) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other indexPath)]
        pub unsafe fn indexPath(&self) -> Id<NSIndexPath, Shared>;

        #[method(isLeaf)]
        pub unsafe fn isLeaf(&self) -> bool;

        #[method_id(@__retain_semantics Other childNodes)]
        pub unsafe fn childNodes(&self) -> Option<Id<NSArray<NSTreeNode>, Shared>>;

        #[method_id(@__retain_semantics Other mutableChildNodes)]
        pub unsafe fn mutableChildNodes(&self) -> Id<NSMutableArray<NSTreeNode>, Owned>;

        #[method_id(@__retain_semantics Other descendantNodeAtIndexPath:)]
        pub unsafe fn descendantNodeAtIndexPath(
            &self,
            indexPath: &NSIndexPath,
        ) -> Option<Id<NSTreeNode, Shared>>;

        #[method_id(@__retain_semantics Other parentNode)]
        pub unsafe fn parentNode(&self) -> Option<Id<NSTreeNode, Shared>>;

        #[method(sortWithSortDescriptors:recursively:)]
        pub unsafe fn sortWithSortDescriptors_recursively(
            &self,
            sortDescriptors: &NSArray<NSSortDescriptor>,
            recursively: bool,
        );
    }
);
