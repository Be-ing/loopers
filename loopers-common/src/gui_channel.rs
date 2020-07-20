use crate::music::{FrameTime, MetricStructure};
use crate::protos::LooperMode;
use crossbeam_channel::{bounded, Sender, Receiver, TrySendError};

#[derive(Copy, Clone, Debug)]
pub struct EngineStateSnapshot {
    pub time: FrameTime,
    pub metric_structure: MetricStructure,
    pub active_looper: u32,
    pub looper_count: usize,
}

#[derive(Copy, Clone, Debug)]
pub enum GuiCommand {
    StateSnapshot(EngineStateSnapshot),
    AddLooper(u32),
    RemoveLooper(u32),

    LooperStateChange(u32, LooperMode),
}

pub struct GuiSender {
    cmd_channel: Option<Sender<GuiCommand>>,
}

pub struct GuiReceiver {
    pub cmd_channel: Receiver<GuiCommand>,
}

impl GuiSender {
    pub fn new() -> (GuiSender, GuiReceiver) {
        let (tx, rx) = bounded(100);

        let sender = GuiSender {
            cmd_channel: Some(tx),
        };

        let receiver = GuiReceiver {
            cmd_channel: rx,
        };

        (sender, receiver)
    }

    pub fn disconnected() -> GuiSender {
        GuiSender {
            cmd_channel: None,
        }
    }

    pub fn send_update(&mut self, cmd: GuiCommand) {
        if let Some(gui_sender) = &self.cmd_channel {
            match gui_sender.try_send(cmd) {
                Ok(_) => {},
                Err(TrySendError::Full(_)) => {
                    warn!("GUI message queue is full");
                },
                Err(TrySendError::Disconnected(_)) => {
                    // TODO: think more about the correct behavior here
                    panic!("GUI message queue is disconnected");
                }
            }
        }

    }
}

impl Clone for GuiSender {
    fn clone(&self) -> Self {
        GuiSender {
            cmd_channel: self.cmd_channel.clone(),
        }
    }
}