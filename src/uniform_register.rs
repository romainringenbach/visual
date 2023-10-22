use std::collections::VecDeque;
use std::sync::Arc;
use vulkano::{
    buffer::{
        allocator::{SubbufferAllocator, SubbufferAllocatorCreateInfo},
        BufferUsage,
    },
    descriptor_set::{
        PersistentDescriptorSet, WriteDescriptorSet,
    },
    memory::allocator::{StandardMemoryAllocator},
};
use vulkano::buffer::BufferContents;
use vulkano::descriptor_set::allocator::DescriptorSetAllocator;
use vulkano::descriptor_set::layout::DescriptorSetLayout;

pub struct UniformRegister {
    allocator : SubbufferAllocator,
    descriptor_writes: VecDeque<WriteDescriptorSet>
}

impl UniformRegister {
    pub fn new(memory_allocator : Arc<StandardMemoryAllocator>, isStorage: bool) -> Self{

        let mut buffer_usage =   BufferUsage::UNIFORM_BUFFER;
        if isStorage {
            buffer_usage = BufferUsage::STORAGE_BUFFER;
        }

        Self {
            allocator : SubbufferAllocator::new(
                memory_allocator.clone(),
                SubbufferAllocatorCreateInfo {
                    buffer_usage,
                    ..Default::default()
                }),
            descriptor_writes: vec![].into(),
        }
    }

    pub fn has_uniform_data(&self) -> bool {
        return self.descriptor_writes.len()>0;
    }

    pub fn register_uniform_data<T: BufferContents>(&mut self, data : T){
        let subbuffer = self.allocator.allocate_sized().unwrap();
        *subbuffer.write().unwrap() = data;
        self.descriptor_writes.push_back(WriteDescriptorSet::buffer(self.descriptor_writes.len() as u32, subbuffer));
    }

    pub fn create_descriptor_set<A>(& mut self, allocator: &A, layout: Arc<DescriptorSetLayout>) -> Arc<PersistentDescriptorSet<A::Alloc>>
        where     A: DescriptorSetAllocator + ?Sized {

        let mut b : Vec<WriteDescriptorSet> = vec![];

        for _i in 0..self.descriptor_writes.len() {
            b.push(self.descriptor_writes.pop_front().unwrap());
        }

        return PersistentDescriptorSet::new(
            allocator,
            layout.clone(),
            b.into_iter()
        ).unwrap();
    }
}