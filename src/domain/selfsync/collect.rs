/// collects channel result into vec
use smol::channel::{Receiver, Sender};
use smol::stream::StreamExt;

pub async fn collect_output<'tasklife, FinalOutput>(
    out_r: Receiver<FinalOutput>,
    quit: Sender<bool>,
    nb_tasks: &'tasklife usize,
) -> Vec<FinalOutput> {
    let result: Vec<FinalOutput> = out_r.collect().await;
    println!(
        "result filled: result len {}, nb_tasks {}",
        result.len(),
        *nb_tasks
    );
    let _ = quit.send(true).await;
    let _ = drop(quit);
    result
}
