//! basic task pipelinening
use smol::{
    Executor,
    channel::{Receiver, unbounded},
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
            println!("starting pipeline loop");
            loop {
                // shutdown channel shouldn't block stuff until arrival
                // so use try_recv
                match quit.try_recv() {
                    Ok(_) => {
                        break;
                    }
                    _ => (),
                }

                // we should wait until input receiver gets something
                match input.recv().await {
                    Ok(task_input) => {
                        let output = task(task_input);
                        let _ = out_sender.send(output).await;
                    }
                    Err(_) => {
                        if input.is_closed() {
                            println!("input closed");
                            break;
                        }
                    }
                }
            }
            drop(out_sender);
            println!("out sender closed");
        })
        .detach();
    // notice that we are not returning the child scope join handler
    // by adding the semicolumn
    return out_receiver;
}
