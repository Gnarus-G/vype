use std::sync::mpsc::channel;
use std::time::Duration;
use vype::actors::{spawn_actor, ActorRef};
use vype::audio::{AudioActor, AudioMsg};
use vype::sources::fakes::FakeAudioSource;

#[test]
fn audio_actor_responds_to_start_recording() {
    let fake_source = FakeAudioSource::new(vec![0.0; 100], 16000, 1);
    let actor = AudioActor::new(fake_source, Duration::from_secs(30));
    let (actor_ref, _handle) = spawn_actor(actor);

    actor_ref.send(AudioMsg::StartRecording).unwrap();

    std::thread::sleep(Duration::from_millis(10));
}

#[test]
fn audio_actor_stop_returns_samples_via_reply() {
    let samples = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    let fake_source = FakeAudioSource::new(samples.clone(), 16000, 1);
    let actor = AudioActor::new(fake_source, Duration::from_secs(30));
    let (actor_ref, _handle) = spawn_actor(actor);

    actor_ref.send(AudioMsg::StartRecording).unwrap();

    let (reply_tx, reply_rx) = channel();
    let reply_ref = ActorRef::new(reply_tx);
    actor_ref
        .send(AudioMsg::StopRecording {
            reply_to: reply_ref,
        })
        .unwrap();

    let result = reply_rx.recv_timeout(Duration::from_secs(1)).unwrap();
    assert_eq!(result, samples);
}
