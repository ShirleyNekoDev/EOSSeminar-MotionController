use dmc::{ClientUpdate};
use tokio::{sync::broadcast::{self, error::SendError}};

pub trait SenderUtils<T> {
    fn broadcast_update(&self, update: T) -> Result<usize, SendError<Vec<T>>>;
    fn broadcast_updates(&self, updates: Vec<T>) -> Result<usize, SendError<Vec<T>>>;
}
impl SenderUtils<ClientUpdate> for  broadcast::Sender<Vec<ClientUpdate>> {
    fn broadcast_update(&self, update: ClientUpdate) -> Result<usize, SendError<Vec<ClientUpdate>>> {
        self.broadcast_updates(vec![update])
    }
    fn broadcast_updates(&self, updates: Vec<ClientUpdate>) -> Result<usize, SendError<Vec<ClientUpdate>>> {
        if self.receiver_count() > 0 {
            self.send(updates)
        } else {
            Ok(0)
        }
    }
}
