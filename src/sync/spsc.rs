use std::sync::{Arc, Condvar, Mutex};
    
pub fn rendezvous_channel<T: Send>() -> (RendezvousSender<T>, RendezvousReceiver<T>) {
    let inner = Arc::new(RendezvousInner {
        data: Mutex::new(LockedState { val: None, sender_alive: true, reciever_alive: true }),
        sender_cond: Condvar::new(),
        receiver_cond: Condvar::new(),
    });
    
    (
        RendezvousSender { inner: inner.clone() },
        RendezvousReceiver { inner },
    )
}
    
#[derive(Debug)]
struct RendezvousInner<T: Send> {
    data: Mutex<LockedState<T>>,
    sender_cond: Condvar,
    receiver_cond: Condvar,
}

#[derive(Debug)]
struct LockedState<T: Send> {
    val: Option<T>,
    sender_alive: bool,
    reciever_alive: bool,
}

#[derive(Debug)]
pub struct RendezvousSender<T: Send> {
    inner: Arc<RendezvousInner<T>>
}

impl<T: Send> RendezvousSender<T> {
    pub fn send(&self, val: T) -> Result<(), T> {
        let mut state = self.inner.data.lock().unwrap();
        
        while state.val.is_some() {
            if !state.reciever_alive { return  Err(val) }

            match self.inner.sender_cond.wait(state) {
                Ok(new_state) => state = new_state,
                Err(_) => return Err(val)
            }
        }

        if !state.reciever_alive {
            return Err(val);
        }

        state.val = Some(val);

        self.inner.receiver_cond.notify_one();

        while state.val.is_some() {
            if !state.reciever_alive { return  Err(state.val.take().unwrap()) }
            match self.inner.sender_cond.wait(state) {
                Ok(new_state) => state = new_state,
                Err(poisoned) => return Err(poisoned.into_inner().val.take().unwrap())
            }
        }

        Ok(())
    }
}
    
impl<T: Send> Drop for RendezvousSender<T> {
    fn drop(&mut self) {
        let mut state = self.inner.data.lock().unwrap();
        state.sender_alive = false;
        drop(state);
        self.inner.receiver_cond.notify_one();
    }
}
    
#[derive(Debug)]
pub struct RendezvousReceiver<T: Send> {
    inner: Arc<RendezvousInner<T>>
}

impl<T: Send> RendezvousReceiver<T> {
    pub fn recv(&self) -> Result<T, ()> {
        let mut state = self.inner.data.lock().unwrap();

        while state.val.is_none() {
            if !state.sender_alive { return Err(()) }
            state = self.inner.receiver_cond.wait(state).map_err(|_| ())? 
        }

        let val = state.val.take().unwrap();
        self.inner.sender_cond.notify_one();

        Ok(val)
    }
}
    
impl<T: Send> Drop for RendezvousReceiver<T> {
    fn drop(&mut self) {
        let mut state = self.inner.data.lock().unwrap();
        state.reciever_alive = false;
        drop(state);
        self.inner.sender_cond.notify_one();
    }
}