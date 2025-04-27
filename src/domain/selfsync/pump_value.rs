//! pump value to input receiver

use smol::{
    Executor,
    channel::{Receiver, TryRecvError, unbounded},
};
use std::marker;

pub fn pump_value_to_channel<
    'tasklife,
    Input1: marker::Send + 'tasklife,
    Input2: marker::Send + 'tasklife + Clone,
>(
    quit: &'tasklife Receiver<bool>,
    input_1: Receiver<Input1>,
    input_2: Input2,
    ex: &mut Executor<'tasklife>,
) -> Receiver<(Input1, Input2)> {
    let (out_sender, out_receiver) = unbounded::<(Input1, Input2)>();
    let _ = ex
        .spawn(async move {
            //
            loop {
                if quit.is_closed() {
                    break;
                }
                match input_1.try_recv() {
                    Ok(j) => {
                        let _ = out_sender.send((j, input_2.clone())).await;
                    }
                    Err(TryRecvError::Closed) => break,
                    Err(TryRecvError::Empty) => (),
                }
            }
        })
        .detach();

    out_receiver
}
