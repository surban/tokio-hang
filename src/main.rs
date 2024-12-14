use std::time::Duration;

use futures::{SinkExt, StreamExt};
use tokio::time::timeout;

async fn send_task(mut tx: futures::channel::mpsc::Sender<u8>) {
    println!("feeding 0");
    tx.feed(0).await.unwrap();
    println!("flushing 0");
    tx.flush().await.unwrap();

    println!("feeding 1");
    tx.feed(1).await.unwrap();
    println!("flushing 1");
    tx.flush().await.unwrap();
}

async fn recv_task(mut rx: futures::channel::mpsc::Receiver<u8>) {
    loop {
        println!("waiting to receive");
        match rx.next().await {
            Some(msg) => println!("received: {msg}"),
            None => break,
        }
    }

    println!("end of receive");
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let qlen = 0;
    let (a_tx, b_rx) = futures::channel::mpsc::channel::<u8>(qlen);

    let task = async move { tokio::join!(send_task(a_tx), recv_task(b_rx)) };
    timeout(Duration::from_secs(60), task).await.unwrap();

    // works without timeout:
    // task.await;
}
