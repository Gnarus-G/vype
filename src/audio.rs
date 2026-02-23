use crate::actors::Actor;
use crate::sources::AudioSource;
use std::sync::mpsc::Receiver;
use std::time::Duration;

pub enum AudioMsg {
    StartRecording,
    StopRecording {
        reply_to: crate::actors::ActorRef<Vec<f32>>,
    },
}

pub struct AudioActor<S: AudioSource + Send + 'static> {
    source: S,
    max_duration: Duration,
}

impl<S: AudioSource + Send + 'static> AudioActor<S> {
    pub fn new(source: S, max_duration: Duration) -> Self {
        Self {
            source,
            max_duration,
        }
    }
}

impl<S: AudioSource + Send + 'static> Actor<AudioMsg> for AudioActor<S> {
    fn run(mut self, receiver: Receiver<AudioMsg>) {
        for msg in receiver.iter() {
            match msg {
                AudioMsg::StartRecording => {
                    let _ = self.source.start();
                }
                AudioMsg::StopRecording { reply_to } => {
                    let samples = self.source.stop();
                    let _ = reply_to.send(samples);
                }
            }
        }
    }
}
