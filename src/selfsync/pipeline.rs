//! basic task pipelinening
use smol::{
    Executor,
    channel::{Receiver, TryRecvError, unbounded},
};
use std::marker;

pub fn add_to_pipeline<
    'tasklife,
    Input: marker::Send + 'tasklife,
    Output: marker::Send + 'tasklife,
>(
    quit: &'tasklife Receiver<bool>,
    task: fn(Input) -> Output,
    input: Receiver<Input>,
    ex: &mut Executor<'tasklife>,
) -> Receiver<Output> {
    let (out_sender, out_receiver) = unbounded::<Output>();

    let _ = ex
        .spawn(async move {
            loop {
                // shutdown channel shouldn't block stuff until arrival
                // so use try_recv
                if quit.is_closed() {
                    break;
                }

                //
                match input.try_recv() {
                    Ok(task_input) => {
                        let output = task(task_input);
                        let _ = out_sender.send(output).await;
                    }
                    Err(TryRecvError::Closed) => break,
                    Err(TryRecvError::Empty) => (),
                }
            }
            drop(out_sender);
        })
        .detach();
    // notice that we are not returning the child scope join handler
    // by adding the semicolumn
    return out_receiver;
}
