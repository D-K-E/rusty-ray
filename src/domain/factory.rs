//! various factory function
use crate::data::imgrad::ImGradientData;

use smol::{
    Executor,
    channel::{Receiver, unbounded},
};

pub fn imgrad_factory<'tasklife>(
    imw: u32,
    imh: u32,
    quit: &'tasklife Receiver<bool>,
    ex: &mut Executor<'tasklife>,
) -> (usize, Receiver<ImGradientData>) {
    let tw = imw.clone();
    let th = imh.clone();
    let nb_tasks: usize = (imw * imh) as usize;
    let (input_send, input_receive) = unbounded::<ImGradientData>();
    let _t = ex
        .spawn(async move {
            for x in 0..imw {
                for y in 0..imh {
                    match quit.try_recv() {
                        Ok(_) => break,
                        _ => (),
                    }
                    let imgrad = ImGradientData::new(x, y, tw, th);
                    let _ = input_send.send(imgrad).await;
                }
            }
            drop(input_send);
            println!("input_send closed");
        })
        .detach();
    println!("finished sending");
    return (nb_tasks, input_receive);
}
