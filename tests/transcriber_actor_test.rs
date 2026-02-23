use std::sync::mpsc::channel;
use std::time::Duration;
use vype::actors::{spawn_actor, ActorRef};
use vype::sources::fakes::MockTranscriber;
use vype::transcriber::{TranscriberActor, TranscriberMsg};

#[test]
fn transcriber_actor_returns_result_via_reply() {
    let mock = MockTranscriber::new("hello world".to_string());
    let actor = TranscriberActor::new(mock);
    let (actor_ref, _handle) = spawn_actor(actor);

    let (reply_tx, reply_rx) = channel();
    let reply_ref = ActorRef::new(reply_tx);
    actor_ref
        .send(TranscriberMsg::Transcribe {
            audio: vec![0.1, 0.2, 0.3],
            reply_to: reply_ref,
        })
        .unwrap();

    let result = reply_rx.recv_timeout(Duration::from_secs(1)).unwrap();
    assert_eq!(result, Ok("hello world".to_string()));
}
