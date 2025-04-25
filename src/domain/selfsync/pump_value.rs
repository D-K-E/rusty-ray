//! pump value to input receiver

use smol::{
    Executor,
    channel::{Receiver, TryRecvError, unbounded},
};
use std::marker;

pub fn pump_value_to_channel<'tasklife, Input_1: marker::Send + 'tasklife, Input_2>(
    quit: &'tasklife Receiver<bool>,
    input_1: Receiver<Input_1>,
    input_2: &'tasklife Input_2,
    ex: &mut Executor<'tasklife>,
) -> Receiver<(&'tasklife Input_1, Input_2)> {
    let (out_sender, out_receiver) = unbounded::<(Input_1, &'tasklife Input_2)>();
    let _ = ex
        .spawn(async move {
            //
            loop {
                if quit.is_closed() {
                    break;
                }
                match input_1.try_recv() {
                    Ok(j) => {
                        let _ = out_sender.send((j, input_2)).await;
                    }
                    Err(TryRecvError::Closed) => break,
                    Err(TryRecvError::Empty) => (),
                }
            }
        })
        .detach();

    out_receiver
}
