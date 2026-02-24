use crate::actors::{Actor, ActorRef};
use crate::audio::AudioMsg;
use crate::sources::KeyboardSink;
use crate::transcriber::TranscriberMsg;
use std::sync::mpsc::Receiver;

pub enum KeyboardMsg {
    PTTPressed,
    PTTReleased,
}

pub struct KeyboardActor {
    audio_ref: ActorRef<AudioMsg>,
    transcriber_ref: ActorRef<TranscriberMsg>,
    keyboard_sink: Box<dyn KeyboardSink + Send>,
}

impl KeyboardActor {
    pub fn new(
        audio_ref: ActorRef<AudioMsg>,
        transcriber_ref: ActorRef<TranscriberMsg>,
        keyboard_sink: Box<dyn KeyboardSink + Send>,
    ) -> Self {
        Self {
            audio_ref,
            transcriber_ref,
            keyboard_sink,
        }
    }
}

impl Actor<KeyboardMsg> for KeyboardActor {
    fn run(mut self, receiver: Receiver<KeyboardMsg>) {
        for msg in receiver.iter() {
            match msg {
                KeyboardMsg::PTTPressed => {
                    let _ = self.audio_ref.send(AudioMsg::StartRecording);
                }
                KeyboardMsg::PTTReleased => {
                    let (reply_tx, reply_rx) = std::sync::mpsc::channel();
                    let _ = self.audio_ref.send(AudioMsg::StopRecording {
                        reply_to: ActorRef::new(reply_tx),
                    });
                    if let Ok(samples) = reply_rx.recv() {
                        let (transcribe_reply_tx, transcribe_reply_rx) = std::sync::mpsc::channel();
                        let _ = self.transcriber_ref.send(TranscriberMsg::Transcribe {
                            audio: samples,
                            reply_to: ActorRef::new(transcribe_reply_tx),
                        });
                        if let Ok(Ok(text)) = transcribe_reply_rx.recv() {
                            self.keyboard_sink.type_text(&text);
                        }
                    }
                }
            }
        }
    }
}
