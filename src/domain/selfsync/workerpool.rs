//! implement a simple worker pool pattern

use crate::domain::selfsync::pipeline::apply_until;
use smol::{
    Executor,
    channel::{Receiver, unbounded},
};
use std::marker;

pub fn spawn_workers<
    'tasklife,
    Input: marker::Send + 'tasklife,
    Output: marker::Send + 'tasklife,
>(
    nb_workers: usize,
    quit: &'tasklife Receiver<bool>,
    task: fn(Input) -> Output,
    input: Receiver<Input>,
    ex: &mut Executor<'tasklife>,
) -> Receiver<Output> {
    let (out_s, out_receiver) = unbounded::<Output>();
    for _i in 0..nb_workers {
        let inp = input.clone();
        let out_sender = out_s.clone();
        let _ = ex
            .spawn(async move {
                apply_until(quit, task, inp, out_sender).await;
            })
            .detach();
        // notice that we are not returning the child scope join handler
        // by adding the semicolumn
    }
    return out_receiver;
}
