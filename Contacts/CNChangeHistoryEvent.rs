//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNChangeHistoryEvent;

    unsafe impl ClassType for CNChangeHistoryEvent {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for CNChangeHistoryEvent {}

unsafe impl NSCopying for CNChangeHistoryEvent {}

unsafe impl CopyingHelper for CNChangeHistoryEvent {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CNChangeHistoryEvent {}

unsafe impl NSSecureCoding for CNChangeHistoryEvent {}

extern_methods!(
    unsafe impl CNChangeHistoryEvent {
        #[method(acceptEventVisitor:)]
        pub unsafe fn acceptEventVisitor(
            &self,
            visitor: &ProtocolObject<dyn CNChangeHistoryEventVisitor>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNChangeHistoryEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNChangeHistoryDropEverythingEvent;

    unsafe impl ClassType for CNChangeHistoryDropEverythingEvent {
        #[inherits(NSObject)]
        type Super = CNChangeHistoryEvent;
    }
);

unsafe impl NSCoding for CNChangeHistoryDropEverythingEvent {}

unsafe impl NSCopying for CNChangeHistoryDropEverythingEvent {}

unsafe impl CopyingHelper for CNChangeHistoryDropEverythingEvent {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CNChangeHistoryDropEverythingEvent {}

unsafe impl NSSecureCoding for CNChangeHistoryDropEverythingEvent {}

extern_methods!(
    unsafe impl CNChangeHistoryDropEverythingEvent {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNChangeHistoryDropEverythingEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNChangeHistoryAddContactEvent;

    unsafe impl ClassType for CNChangeHistoryAddContactEvent {
        #[inherits(NSObject)]
        type Super = CNChangeHistoryEvent;
    }
);

unsafe impl NSCoding for CNChangeHistoryAddContactEvent {}

unsafe impl NSCopying for CNChangeHistoryAddContactEvent {}

unsafe impl CopyingHelper for CNChangeHistoryAddContactEvent {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CNChangeHistoryAddContactEvent {}

unsafe impl NSSecureCoding for CNChangeHistoryAddContactEvent {}

extern_methods!(
    unsafe impl CNChangeHistoryAddContactEvent {
        #[cfg(feature = "CNContact")]
        #[method_id(@__retain_semantics Other contact)]
        pub unsafe fn contact(&self) -> Retained<CNContact>;

        #[method_id(@__retain_semantics Other containerIdentifier)]
        pub unsafe fn containerIdentifier(&self) -> Option<Retained<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNChangeHistoryAddContactEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNChangeHistoryUpdateContactEvent;

    unsafe impl ClassType for CNChangeHistoryUpdateContactEvent {
        #[inherits(NSObject)]
        type Super = CNChangeHistoryEvent;
    }
);

unsafe impl NSCoding for CNChangeHistoryUpdateContactEvent {}

unsafe impl NSCopying for CNChangeHistoryUpdateContactEvent {}

unsafe impl CopyingHelper for CNChangeHistoryUpdateContactEvent {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CNChangeHistoryUpdateContactEvent {}

unsafe impl NSSecureCoding for CNChangeHistoryUpdateContactEvent {}

extern_methods!(
    unsafe impl CNChangeHistoryUpdateContactEvent {
        #[cfg(feature = "CNContact")]
        #[method_id(@__retain_semantics Other contact)]
        pub unsafe fn contact(&self) -> Retained<CNContact>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNChangeHistoryUpdateContactEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNChangeHistoryDeleteContactEvent;

    unsafe impl ClassType for CNChangeHistoryDeleteContactEvent {
        #[inherits(NSObject)]
        type Super = CNChangeHistoryEvent;
    }
);

unsafe impl NSCoding for CNChangeHistoryDeleteContactEvent {}

unsafe impl NSCopying for CNChangeHistoryDeleteContactEvent {}

unsafe impl CopyingHelper for CNChangeHistoryDeleteContactEvent {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CNChangeHistoryDeleteContactEvent {}

unsafe impl NSSecureCoding for CNChangeHistoryDeleteContactEvent {}

extern_methods!(
    unsafe impl CNChangeHistoryDeleteContactEvent {
        #[method_id(@__retain_semantics Other contactIdentifier)]
        pub unsafe fn contactIdentifier(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNChangeHistoryDeleteContactEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNChangeHistoryAddGroupEvent;

    unsafe impl ClassType for CNChangeHistoryAddGroupEvent {
        #[inherits(NSObject)]
        type Super = CNChangeHistoryEvent;
    }
);

unsafe impl NSCoding for CNChangeHistoryAddGroupEvent {}

unsafe impl NSCopying for CNChangeHistoryAddGroupEvent {}

unsafe impl CopyingHelper for CNChangeHistoryAddGroupEvent {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CNChangeHistoryAddGroupEvent {}

unsafe impl NSSecureCoding for CNChangeHistoryAddGroupEvent {}

extern_methods!(
    unsafe impl CNChangeHistoryAddGroupEvent {
        #[cfg(feature = "CNGroup")]
        #[method_id(@__retain_semantics Other group)]
        pub unsafe fn group(&self) -> Retained<CNGroup>;

        #[method_id(@__retain_semantics Other containerIdentifier)]
        pub unsafe fn containerIdentifier(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNChangeHistoryAddGroupEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNChangeHistoryUpdateGroupEvent;

    unsafe impl ClassType for CNChangeHistoryUpdateGroupEvent {
        #[inherits(NSObject)]
        type Super = CNChangeHistoryEvent;
    }
);

unsafe impl NSCoding for CNChangeHistoryUpdateGroupEvent {}

unsafe impl NSCopying for CNChangeHistoryUpdateGroupEvent {}

unsafe impl CopyingHelper for CNChangeHistoryUpdateGroupEvent {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CNChangeHistoryUpdateGroupEvent {}

unsafe impl NSSecureCoding for CNChangeHistoryUpdateGroupEvent {}

extern_methods!(
    unsafe impl CNChangeHistoryUpdateGroupEvent {
        #[cfg(feature = "CNGroup")]
        #[method_id(@__retain_semantics Other group)]
        pub unsafe fn group(&self) -> Retained<CNGroup>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNChangeHistoryUpdateGroupEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNChangeHistoryDeleteGroupEvent;

    unsafe impl ClassType for CNChangeHistoryDeleteGroupEvent {
        #[inherits(NSObject)]
        type Super = CNChangeHistoryEvent;
    }
);

unsafe impl NSCoding for CNChangeHistoryDeleteGroupEvent {}

unsafe impl NSCopying for CNChangeHistoryDeleteGroupEvent {}

unsafe impl CopyingHelper for CNChangeHistoryDeleteGroupEvent {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CNChangeHistoryDeleteGroupEvent {}

unsafe impl NSSecureCoding for CNChangeHistoryDeleteGroupEvent {}

extern_methods!(
    unsafe impl CNChangeHistoryDeleteGroupEvent {
        #[method_id(@__retain_semantics Other groupIdentifier)]
        pub unsafe fn groupIdentifier(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNChangeHistoryDeleteGroupEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNChangeHistoryAddMemberToGroupEvent;

    unsafe impl ClassType for CNChangeHistoryAddMemberToGroupEvent {
        #[inherits(NSObject)]
        type Super = CNChangeHistoryEvent;
    }
);

unsafe impl NSCoding for CNChangeHistoryAddMemberToGroupEvent {}

unsafe impl NSCopying for CNChangeHistoryAddMemberToGroupEvent {}

unsafe impl CopyingHelper for CNChangeHistoryAddMemberToGroupEvent {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CNChangeHistoryAddMemberToGroupEvent {}

unsafe impl NSSecureCoding for CNChangeHistoryAddMemberToGroupEvent {}

extern_methods!(
    unsafe impl CNChangeHistoryAddMemberToGroupEvent {
        #[cfg(feature = "CNContact")]
        #[method_id(@__retain_semantics Other member)]
        pub unsafe fn member(&self) -> Retained<CNContact>;

        #[cfg(feature = "CNGroup")]
        #[method_id(@__retain_semantics Other group)]
        pub unsafe fn group(&self) -> Retained<CNGroup>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNChangeHistoryAddMemberToGroupEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNChangeHistoryRemoveMemberFromGroupEvent;

    unsafe impl ClassType for CNChangeHistoryRemoveMemberFromGroupEvent {
        #[inherits(NSObject)]
        type Super = CNChangeHistoryEvent;
    }
);

unsafe impl NSCoding for CNChangeHistoryRemoveMemberFromGroupEvent {}

unsafe impl NSCopying for CNChangeHistoryRemoveMemberFromGroupEvent {}

unsafe impl CopyingHelper for CNChangeHistoryRemoveMemberFromGroupEvent {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CNChangeHistoryRemoveMemberFromGroupEvent {}

unsafe impl NSSecureCoding for CNChangeHistoryRemoveMemberFromGroupEvent {}

extern_methods!(
    unsafe impl CNChangeHistoryRemoveMemberFromGroupEvent {
        #[cfg(feature = "CNContact")]
        #[method_id(@__retain_semantics Other member)]
        pub unsafe fn member(&self) -> Retained<CNContact>;

        #[cfg(feature = "CNGroup")]
        #[method_id(@__retain_semantics Other group)]
        pub unsafe fn group(&self) -> Retained<CNGroup>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNChangeHistoryRemoveMemberFromGroupEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNChangeHistoryAddSubgroupToGroupEvent;

    unsafe impl ClassType for CNChangeHistoryAddSubgroupToGroupEvent {
        #[inherits(NSObject)]
        type Super = CNChangeHistoryEvent;
    }
);

unsafe impl NSCoding for CNChangeHistoryAddSubgroupToGroupEvent {}

unsafe impl NSCopying for CNChangeHistoryAddSubgroupToGroupEvent {}

unsafe impl CopyingHelper for CNChangeHistoryAddSubgroupToGroupEvent {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CNChangeHistoryAddSubgroupToGroupEvent {}

unsafe impl NSSecureCoding for CNChangeHistoryAddSubgroupToGroupEvent {}

extern_methods!(
    unsafe impl CNChangeHistoryAddSubgroupToGroupEvent {
        #[cfg(feature = "CNGroup")]
        #[method_id(@__retain_semantics Other subgroup)]
        pub unsafe fn subgroup(&self) -> Retained<CNGroup>;

        #[cfg(feature = "CNGroup")]
        #[method_id(@__retain_semantics Other group)]
        pub unsafe fn group(&self) -> Retained<CNGroup>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNChangeHistoryAddSubgroupToGroupEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNChangeHistoryRemoveSubgroupFromGroupEvent;

    unsafe impl ClassType for CNChangeHistoryRemoveSubgroupFromGroupEvent {
        #[inherits(NSObject)]
        type Super = CNChangeHistoryEvent;
    }
);

unsafe impl NSCoding for CNChangeHistoryRemoveSubgroupFromGroupEvent {}

unsafe impl NSCopying for CNChangeHistoryRemoveSubgroupFromGroupEvent {}

unsafe impl CopyingHelper for CNChangeHistoryRemoveSubgroupFromGroupEvent {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CNChangeHistoryRemoveSubgroupFromGroupEvent {}

unsafe impl NSSecureCoding for CNChangeHistoryRemoveSubgroupFromGroupEvent {}

extern_methods!(
    unsafe impl CNChangeHistoryRemoveSubgroupFromGroupEvent {
        #[cfg(feature = "CNGroup")]
        #[method_id(@__retain_semantics Other subgroup)]
        pub unsafe fn subgroup(&self) -> Retained<CNGroup>;

        #[cfg(feature = "CNGroup")]
        #[method_id(@__retain_semantics Other group)]
        pub unsafe fn group(&self) -> Retained<CNGroup>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNChangeHistoryRemoveSubgroupFromGroupEvent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait CNChangeHistoryEventVisitor: NSObjectProtocol {
        #[method(visitDropEverythingEvent:)]
        unsafe fn visitDropEverythingEvent(&self, event: &CNChangeHistoryDropEverythingEvent);

        #[method(visitAddContactEvent:)]
        unsafe fn visitAddContactEvent(&self, event: &CNChangeHistoryAddContactEvent);

        #[method(visitUpdateContactEvent:)]
        unsafe fn visitUpdateContactEvent(&self, event: &CNChangeHistoryUpdateContactEvent);

        #[method(visitDeleteContactEvent:)]
        unsafe fn visitDeleteContactEvent(&self, event: &CNChangeHistoryDeleteContactEvent);

        #[optional]
        #[method(visitAddGroupEvent:)]
        unsafe fn visitAddGroupEvent(&self, event: &CNChangeHistoryAddGroupEvent);

        #[optional]
        #[method(visitUpdateGroupEvent:)]
        unsafe fn visitUpdateGroupEvent(&self, event: &CNChangeHistoryUpdateGroupEvent);

        #[optional]
        #[method(visitDeleteGroupEvent:)]
        unsafe fn visitDeleteGroupEvent(&self, event: &CNChangeHistoryDeleteGroupEvent);

        #[optional]
        #[method(visitAddMemberToGroupEvent:)]
        unsafe fn visitAddMemberToGroupEvent(&self, event: &CNChangeHistoryAddMemberToGroupEvent);

        #[optional]
        #[method(visitRemoveMemberFromGroupEvent:)]
        unsafe fn visitRemoveMemberFromGroupEvent(
            &self,
            event: &CNChangeHistoryRemoveMemberFromGroupEvent,
        );

        #[optional]
        #[method(visitAddSubgroupToGroupEvent:)]
        unsafe fn visitAddSubgroupToGroupEvent(
            &self,
            event: &CNChangeHistoryAddSubgroupToGroupEvent,
        );

        #[optional]
        #[method(visitRemoveSubgroupFromGroupEvent:)]
        unsafe fn visitRemoveSubgroupFromGroupEvent(
            &self,
            event: &CNChangeHistoryRemoveSubgroupFromGroupEvent,
        );
    }

    unsafe impl ProtocolType for dyn CNChangeHistoryEventVisitor {}
);
