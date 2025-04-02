//! basic task pipelinening
use smol::channel::Receiver;
use smol::channel::Sender;
use smol::channel::unbounded;
use smol::{Executor};
use std::marker;

async fn apply_send_task<Input: marker::Send, Output: marker::Send>(
    task: &fn(Input) -> Output,
    input: &Receiver<Input>,
    out: &Sender<Output>,
) {
    match input.try_recv() {
        Ok(task_input) => {
            let output = task(task_input);
            out.send(output).await;
        }
        Err(_) => {}
    }
}

pub fn add_to_pipeline<'tasklife, Input: marker::Send, Output: marker::Send + 'tasklife>(
    quit: &'tasklife Receiver<bool>,
    task: fn(Input) -> Output,
    input: &'tasklife Receiver<Input>,
    ex: &mut Executor<'tasklife>,
) -> Receiver<Output> {
    let (out_sender, out_receiver) = unbounded::<Output>();

    let t = ex.spawn(async move {
        loop {
            match quit.try_recv() {
                Ok(_) => {
                    break;
                }
                Err(_) => apply_send_task(&task, input, &out_sender).await,
            }
        }
        drop(out_sender);
    });
    // notice that we are not returning the child scope join handler
    // by adding the semicolumn
    return out_receiver;
}
