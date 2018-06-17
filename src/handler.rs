use crossbeam_channel as channel;
// use crossbeam;
use ChannelData;

lazy_static! {    
    // pub static ref CHANNELS: (channel::Sender<u32>, channel::Receiver<u32>) = {
    pub static ref CHANNELS: (channel::Sender<ChannelData>, channel::Receiver<ChannelData>) = {
        let (sender, receiver) = channel::unbounded::<ChannelData>();
        (sender, receiver)
    };
}

use binding;

pub struct Handler {
    // workers: HashMap<i32, worker::Worker<T>>,
    pub receiver: channel::Receiver<ChannelData>,
    pub sender: channel::Sender<ChannelData>,
}

pub fn new() -> Handler {
    let sender = CHANNELS.0.clone();
    let receiver = CHANNELS.1.clone();
    let h = Handler{
        // workers: HashMap::new(),
        receiver: receiver,
        sender: sender,
    };
    h
}


impl Handler {
    pub fn init(&mut self) {
        unsafe {
            binding::v8_init();
        };
    }

    pub fn recv(&mut self) {
        let receiver = CHANNELS.1.clone();
        for _m in receiver {
            let _ch_data: ChannelData = _m;
            // let _worker = self.workers.get_mut(&_ch_data.worker_id);
            // self.workers.get_mut(&_ch_data.worker_id);
        };
    }
}