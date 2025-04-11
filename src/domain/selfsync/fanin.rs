//! fan-in pattern

use smol::{
    Executor,
    channel::{Receiver, TryRecvError, unbounded},
    stream::{Stream, StreamExt},
};

use std::marker;

use crate::domain::selfsync::waitgroup::WaitGroup;

pub fn fan_in<'tasklife, Input: marker::Send + 'tasklife, Output: marker::Send + 'tasklife>(
    quit: &'tasklife Receiver<bool>,
    inputs: Vec<Receiver<Input>>,
    ex: &mut Executor<'tasklife>,
) -> Receiver<Output> {
    let (out_sender, out_receiver) = unbounded::<Output>();
    let mut wg = WaitGroup::new();
    wg.add_member(inputs.len() as i16);
    for (i, chnl) in inputs.iter().enumerate() {
        let _ = ex
            .spawn(async {
                //
                loop {
                    if quit.is_closed() {
                        break;
                    }
                    match chnl.try_recv() {
                        Ok(j) => {
                            let _ = out_sender.send(j).await;
                        }
                        Err(TryRecvError::Closed) => break,
                        Err(TryRecvError::Empty) => (),
                    }
                }
                wg.done();
            })
            .detach();
    }
    let _ = ex
        .spawn(async {
            wg.wait();
            drop(out_sender);
        })
        .detach();
    out_receiver
}
