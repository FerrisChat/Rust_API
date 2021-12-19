use tokio_tungstenite::tungstenite::protocol::CloseFrame;

#[derive(debug)]
pub enum WebsocketError {
    Closed(Option<CloseFrame<'static>>),
}
