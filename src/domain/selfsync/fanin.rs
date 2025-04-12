//! fan-in pattern

use smol::{
    Executor,
    channel::{Receiver, TryRecvError, unbounded},
};

use std::marker;


pub fn fan_in<'tasklife, Input: marker::Send + 'tasklife>(
    quit: &'tasklife Receiver<bool>,
    inputs: Vec<Receiver<Input>>,
    ex: &mut Executor<'tasklife>,
) -> Receiver<Input> {
    let (out_sender, out_receiver) = unbounded::<Input>();
    for chnl in inputs {
        let o_s = out_sender.clone();
        let _ = ex
            .spawn(async move {
                //
                loop {
                    if quit.is_closed() {
                        break;
                    }
                    match chnl.try_recv() {
                        Ok(j) => {
                            let _ = o_s.send(j).await;
                        }
                        Err(TryRecvError::Closed) => break,
                        Err(TryRecvError::Empty) => (),
                    }
                }
                drop(o_s);
            })
            .detach();
    }
    drop(out_sender);
    
    out_receiver
}
