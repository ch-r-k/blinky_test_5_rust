use crate::application_layer::button::Button;
use crate::device_layer::user_input::UserInput;
use embassy_executor::task;

/// Button monitoring task - delegates button handling to Button struct
#[task]
pub async fn button_task(input: UserInput<'static>) {
    let mut button = Button::new(input);

    loop {
        button.handle_press().await;
    }
}
