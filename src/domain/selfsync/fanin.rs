//! fan-in pattern

use smol::{
    Executor,
    channel::{Receiver, TryRecvError, unbounded},
};

use std::marker;

pub fn fan_in_v1<'tasklife, Input: marker::Send + 'tasklife>(
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

pub fn fan_in_tuple<
    'tasklife,
    Input1: marker::Send + 'tasklife,
    Input2: marker::Send + 'tasklife,
>(
    quit: &'tasklife Receiver<bool>,
    input_1: Receiver<Input1>,
    input_2: Receiver<Input2>,
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
                    Ok(j) => match input_2.try_recv() {
                        Ok(i) => {
                            let _ = out_sender.send((j, i)).await;
                        }
                        Err(TryRecvError::Closed) => break,
                        Err(TryRecvError::Empty) => (),
                    },
                    Err(TryRecvError::Closed) => break,
                    Err(TryRecvError::Empty) => (),
                }
            }
        })
        .detach();

    out_receiver
}
