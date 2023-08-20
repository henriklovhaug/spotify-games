use chrono::Duration;
use tokio::time::sleep;

use crate::{store::Store, ChannelMessage};

pub async fn play_sixminutes(store: Store) {
    todo!("Implement play playlist");
    six_minutes_timer(store).await;
}

async fn six_minutes_timer(store: Store) -> () {
    let message = ChannelMessage::new("six minutes".into(), "Game over".into());
    let tx = store.get_sender();

    sleep(Duration::minutes(6).to_std().unwrap()).await;
    let _ = tx.send(message);
}
