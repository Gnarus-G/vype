use crate::actors::Actor;
use crate::sources::SourceError;
use crate::sources::Transcriber;
use std::sync::mpsc::Receiver;

pub enum TranscriberMsg {
    Transcribe {
        audio: Vec<f32>,
        reply_to: crate::actors::ActorRef<Result<String, SourceError>>,
    },
}

pub struct TranscriberActor<T: Transcriber + Send + 'static> {
    transcriber: T,
}

impl<T: Transcriber + Send + 'static> TranscriberActor<T> {
    pub fn new(transcriber: T) -> Self {
        Self { transcriber }
    }
}

impl<T: Transcriber + Send + 'static> Actor<TranscriberMsg> for TranscriberActor<T> {
    fn run(self, receiver: Receiver<TranscriberMsg>) {
        for msg in receiver.iter() {
            match msg {
                TranscriberMsg::Transcribe { audio, reply_to } => {
                    let result = self.transcriber.transcribe(&audio);
                    let _ = reply_to.send(result);
                }
            }
        }
    }
}
