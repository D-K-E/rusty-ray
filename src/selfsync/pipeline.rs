//! basic task pipelinening
use smol::Executor;
use smol::channel::Receiver;
use smol::channel::Sender;
use smol::channel::unbounded;
use std::marker;

async fn apply_send_task<Input: marker::Send, Output: marker::Send>(
    task: fn(Input) -> Output,
    input: &Receiver<Input>,
    out: &Sender<Output>,
) {
}

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

    let t = ex
        .spawn(async move {
            println!("starting pipeline loop");
            loop {
                match quit.try_recv() {
                    Ok(msg) => {
                        break;
                    }
                    _ => (),
                }
                match input.recv().await {
                    Ok(task_input) => {
                        let output = task(task_input);
                        let _ = out_sender.send(output).await;
                    }
                    Err(err) => {
                        if input.is_closed() {
                            println!("input closed");
                            break;
                        }
                    }
                }
            }
            out_sender.close();
            println!("out sender closed");
            drop(out_sender);
        })
        .detach();
    // notice that we are not returning the child scope join handler
    // by adding the semicolumn
    return out_receiver;
}
