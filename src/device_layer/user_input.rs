use crate::device_layer_abstraction::icb_user_input_press::IcbUserInput;
use crate::hardware_layer_abstraction::i_gpio_input::IGpioInput;
use crate::hardware_layer_abstraction::icb_gpio_input::IcbGpioInput;

pub struct UserInput<InputType: IGpioInput> {
    input: InputType,
}

pub struct UserInputPressCallbackAdapter<CallbackType: IcbUserInput> {
    callback: CallbackType,
}

impl<CallbackType: IcbUserInput> UserInputPressCallbackAdapter<CallbackType> {
    pub fn new(callback: CallbackType) -> Self {
        Self { callback }
    }
}

impl<CallbackType: IcbUserInput> IcbGpioInput
    for UserInputPressCallbackAdapter<CallbackType>
{
    fn on_press(&mut self) {
        self.callback.on_user_input_press();
    }
}

impl<InputType: IGpioInput> UserInput<InputType> {
    pub fn new(input: InputType) -> Self {
        Self { input }
    }

    pub fn into_inner(self) -> InputType {
        self.input
    }
}

