//! basic task pipelinening
use crossbeam_channel::Receiver;
use crossbeam_channel::unbounded;

use std::thread;

pub fn add_to_pipeline<Input, Output>(
    quit: &Receiver<bool>,
    task: fn(Input) -> Output,
    input: &Receiver<Input>,
) -> Receiver<Output> {
    let (out_sender, out_receiver) = unbounded::<Output>();
}
