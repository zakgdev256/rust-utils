use std::marker::PhantomData;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhantomUnSend(PhantomData<*const ()>);
unsafe impl Sync for PhantomUnSend {}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhantomUnSync(PhantomData<*const ()>);
unsafe impl Send for PhantomUnSync {}

pub fn is_send<T: Send>() {}
pub fn is_sync<T: Sync>() {}