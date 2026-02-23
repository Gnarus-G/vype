use std::sync::mpsc::Receiver;
use std::time::Duration;
use vype::actors::{spawn_actor, Actor, ActorRef};

#[test]
fn actor_ref_can_send_and_receive_messages() {
    let (tx, rx) = std::sync::mpsc::channel::<String>();

    let actor_ref = ActorRef::new(tx);

    actor_ref.send("hello".to_string()).unwrap();
    actor_ref.send("world".to_string()).unwrap();

    assert_eq!(rx.recv().unwrap(), "hello");
    assert_eq!(rx.recv().unwrap(), "world");
}

#[test]
fn actor_ref_can_be_cloned() {
    let (tx, rx) = std::sync::mpsc::channel::<i32>();

    let actor_ref1 = ActorRef::new(tx);
    let actor_ref2 = actor_ref1.clone();

    actor_ref1.send(1).unwrap();
    actor_ref2.send(2).unwrap();

    assert_eq!(rx.recv().unwrap(), 1);
    assert_eq!(rx.recv().unwrap(), 2);
}

enum TestMsg {
    Increment,
    GetValue(ActorRef<i32>),
}

struct CounterActor {
    count: i32,
}

impl CounterActor {
    fn new() -> Self {
        Self { count: 0 }
    }
}

impl Actor<TestMsg> for CounterActor {
    fn run(mut self, receiver: Receiver<TestMsg>) {
        for msg in receiver.iter() {
            match msg {
                TestMsg::Increment => self.count += 1,
                TestMsg::GetValue(reply) => {
                    let _ = reply.send(self.count);
                }
            }
        }
    }
}

#[test]
fn spawn_actor_returns_ref_and_handle() {
    let (actor_ref, handle) = spawn_actor(CounterActor::new());

    actor_ref.send(TestMsg::Increment).unwrap();
    actor_ref.send(TestMsg::Increment).unwrap();

    let (reply_tx, reply_rx) = std::sync::mpsc::channel();
    let reply_ref = ActorRef::new(reply_tx);
    actor_ref.send(TestMsg::GetValue(reply_ref)).unwrap();

    let value = reply_rx.recv_timeout(Duration::from_secs(1)).unwrap();
    assert_eq!(value, 2);

    drop(actor_ref);
    handle.join().unwrap();
}
