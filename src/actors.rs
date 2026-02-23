use std::sync::mpsc::{Receiver, Sender};
use std::thread::{self, JoinHandle};

pub struct ActorRef<M> {
    sender: Sender<M>,
}

impl<M> ActorRef<M> {
    pub fn new(sender: Sender<M>) -> Self {
        Self { sender }
    }

    pub fn send(&self, msg: M) -> Result<(), std::sync::mpsc::SendError<M>> {
        self.sender.send(msg)
    }
}

impl<M> Clone for ActorRef<M> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}

pub trait Actor<M> {
    fn run(self, receiver: Receiver<M>);
}

pub fn spawn_actor<M, A>(actor: A) -> (ActorRef<M>, JoinHandle<()>)
where
    M: Send + 'static,
    A: Actor<M> + Send + 'static,
{
    let (tx, rx) = std::sync::mpsc::channel();
    let actor_ref = ActorRef::new(tx);
    let handle = thread::spawn(move || actor.run(rx));
    (actor_ref, handle)
}
