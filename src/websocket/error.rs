use tokio_tungstenite::tungstenite::protocol::CloseFrame;

#[derive(Debug)]
pub enum WebsocketError {
    Closed(Option<CloseFrame<'static>>),
}
