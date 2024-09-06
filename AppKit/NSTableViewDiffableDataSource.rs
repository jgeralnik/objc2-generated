//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTableColumn",
    feature = "NSTableView",
    feature = "NSView",
    feature = "block2"
))]
pub type NSTableViewDiffableDataSourceCellProvider = *mut block2::Block<
    dyn Fn(
        NonNull<NSTableView>,
        NonNull<NSTableColumn>,
        NSInteger,
        NonNull<AnyObject>,
    ) -> NonNull<NSView>,
>;

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTableRowView",
    feature = "NSTableView",
    feature = "NSView",
    feature = "block2"
))]
pub type NSTableViewDiffableDataSourceRowProvider = *mut block2::Block<
    dyn Fn(NonNull<NSTableView>, NSInteger, NonNull<AnyObject>) -> NonNull<NSTableRowView>,
>;

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSTableView",
    feature = "NSView",
    feature = "block2"
))]
pub type NSTableViewDiffableDataSourceSectionHeaderViewProvider = *mut block2::Block<
    dyn Fn(NonNull<NSTableView>, NSInteger, NonNull<AnyObject>) -> NonNull<NSView>,
>;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTableViewDiffableDataSource<
        SectionIdentifierType: ?Sized = AnyObject,
        ItemIdentifierType: ?Sized = AnyObject,
    > {
        __superclass: NSObject,
        _inner0: PhantomData<*mut SectionIdentifierType>,
        _inner1: PhantomData<*mut ItemIdentifierType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<SectionIdentifierType: ?Sized + Message, ItemIdentifierType: ?Sized + Message>
        ClassType for NSTableViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
    {
        type Super = NSObject;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }
    }
);

unsafe impl<SectionIdentifierType: ?Sized, ItemIdentifierType: ?Sized> NSObjectProtocol
    for NSTableViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
{
}

#[cfg(feature = "NSTableView")]
unsafe impl<SectionIdentifierType: ?Sized, ItemIdentifierType: ?Sized> NSTableViewDataSource
    for NSTableViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
{
}

extern_methods!(
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message>
        NSTableViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
    {
        #[cfg(all(
            feature = "NSControl",
            feature = "NSResponder",
            feature = "NSTableColumn",
            feature = "NSTableView",
            feature = "NSView",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Init initWithTableView:cellProvider:)]
        pub unsafe fn initWithTableView_cellProvider(
            this: Allocated<Self>,
            table_view: &NSTableView,
            cell_provider: NSTableViewDiffableDataSourceCellProvider,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "NSDiffableDataSource")]
        #[method_id(@__retain_semantics Other snapshot)]
        pub unsafe fn snapshot(
            &self,
        ) -> Retained<NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>>;

        #[cfg(feature = "NSDiffableDataSource")]
        #[method(applySnapshot:animatingDifferences:)]
        pub unsafe fn applySnapshot_animatingDifferences(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
            animating_differences: bool,
        );

        #[cfg(all(feature = "NSDiffableDataSource", feature = "block2"))]
        #[method(applySnapshot:animatingDifferences:completion:)]
        pub unsafe fn applySnapshot_animatingDifferences_completion(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
            animating_differences: bool,
            completion: Option<&block2::Block<dyn Fn()>>,
        );

        #[method_id(@__retain_semantics Other itemIdentifierForRow:)]
        pub unsafe fn itemIdentifierForRow(
            &self,
            row: NSInteger,
        ) -> Option<Retained<ItemIdentifierType>>;

        #[method(rowForItemIdentifier:)]
        pub unsafe fn rowForItemIdentifier(&self, identifier: &ItemIdentifierType) -> NSInteger;

        #[method_id(@__retain_semantics Other sectionIdentifierForRow:)]
        pub unsafe fn sectionIdentifierForRow(
            &self,
            row: NSInteger,
        ) -> Option<Retained<SectionIdentifierType>>;

        #[method(rowForSectionIdentifier:)]
        pub unsafe fn rowForSectionIdentifier(
            &self,
            identifier: &SectionIdentifierType,
        ) -> NSInteger;

        #[cfg(all(
            feature = "NSControl",
            feature = "NSResponder",
            feature = "NSTableRowView",
            feature = "NSTableView",
            feature = "NSView",
            feature = "block2"
        ))]
        #[method(rowViewProvider)]
        pub unsafe fn rowViewProvider(
            &self,
            mtm: MainThreadMarker,
        ) -> NSTableViewDiffableDataSourceRowProvider;

        #[cfg(all(
            feature = "NSControl",
            feature = "NSResponder",
            feature = "NSTableRowView",
            feature = "NSTableView",
            feature = "NSView",
            feature = "block2"
        ))]
        #[method(setRowViewProvider:)]
        pub unsafe fn setRowViewProvider(
            &self,
            row_view_provider: NSTableViewDiffableDataSourceRowProvider,
        );

        #[cfg(all(
            feature = "NSControl",
            feature = "NSResponder",
            feature = "NSTableView",
            feature = "NSView",
            feature = "block2"
        ))]
        #[method(sectionHeaderViewProvider)]
        pub unsafe fn sectionHeaderViewProvider(
            &self,
            mtm: MainThreadMarker,
        ) -> NSTableViewDiffableDataSourceSectionHeaderViewProvider;

        #[cfg(all(
            feature = "NSControl",
            feature = "NSResponder",
            feature = "NSTableView",
            feature = "NSView",
            feature = "block2"
        ))]
        #[method(setSectionHeaderViewProvider:)]
        pub unsafe fn setSectionHeaderViewProvider(
            &self,
            section_header_view_provider: NSTableViewDiffableDataSourceSectionHeaderViewProvider,
        );

        #[cfg(feature = "NSTableView")]
        #[method(defaultRowAnimation)]
        pub unsafe fn defaultRowAnimation(&self) -> NSTableViewAnimationOptions;

        #[cfg(feature = "NSTableView")]
        #[method(setDefaultRowAnimation:)]
        pub unsafe fn setDefaultRowAnimation(
            &self,
            default_row_animation: NSTableViewAnimationOptions,
        );
    }
);
