//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

// NS_TYPED_ENUM
pub type MTKModelError = NSString;

extern "C" {
    pub static MTKModelErrorDomain: &'static MTKModelError;
}

extern "C" {
    pub static MTKModelErrorKey: &'static MTKModelError;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTKMeshBufferAllocator;

    unsafe impl ClassType for MTKMeshBufferAllocator {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for MTKMeshBufferAllocator {}

extern_methods!(
    unsafe impl MTKMeshBufferAllocator {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other device)]
        pub unsafe fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTKMeshBufferAllocator {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTKMeshBuffer;

    unsafe impl ClassType for MTKMeshBuffer {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for MTKMeshBuffer {}

unsafe impl CopyingHelper for MTKMeshBuffer {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTKMeshBuffer {}

extern_methods!(
    unsafe impl MTKMeshBuffer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(length)]
        pub unsafe fn length(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other allocator)]
        pub unsafe fn allocator(&self) -> Retained<MTKMeshBufferAllocator>;

        #[method_id(@__retain_semantics Other buffer)]
        pub unsafe fn buffer(&self) -> Retained<ProtocolObject<dyn MTLBuffer>>;

        #[method(offset)]
        pub unsafe fn offset(&self) -> NSUInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTKMeshBuffer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTKSubmesh;

    unsafe impl ClassType for MTKSubmesh {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for MTKSubmesh {}

extern_methods!(
    unsafe impl MTKSubmesh {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(primitiveType)]
        pub unsafe fn primitiveType(&self) -> MTLPrimitiveType;

        #[method(indexType)]
        pub unsafe fn indexType(&self) -> MTLIndexType;

        #[method_id(@__retain_semantics Other indexBuffer)]
        pub unsafe fn indexBuffer(&self) -> Retained<MTKMeshBuffer>;

        #[method(indexCount)]
        pub unsafe fn indexCount(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other mesh)]
        pub unsafe fn mesh(&self) -> Option<Retained<MTKMesh>>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTKSubmesh {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTKMesh;

    unsafe impl ClassType for MTKMesh {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for MTKMesh {}

extern_methods!(
    unsafe impl MTKMesh {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other vertexBuffers)]
        pub unsafe fn vertexBuffers(&self) -> Retained<NSArray<MTKMeshBuffer>>;

        #[method_id(@__retain_semantics Other submeshes)]
        pub unsafe fn submeshes(&self) -> Retained<NSArray<MTKSubmesh>>;

        #[method(vertexCount)]
        pub unsafe fn vertexCount(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTKMesh {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
