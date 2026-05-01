use rtic_sync::channel::{Receiver, Sender, TrySendError};

pub type BusSender<'a, T, const N: usize> = Sender<'a, T, N>;
pub type BusReceiver<'a, T, const N: usize> = Receiver<'a, T, N>;

pub fn try_publish<T, const N: usize>(
    sender: &mut Sender<'_, T, N>,
    message: T,
) -> Result<(), TrySendError<T>> {
    sender.try_send(message)
}