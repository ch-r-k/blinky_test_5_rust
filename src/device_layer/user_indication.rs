use crate::device_layer_abstraction::i_ui::IUi;
use crate::hardware_layer_abstraction::i_smart_led_bus::ISmartLedBus;
use crate::hardware_layer_abstraction::i_smart_led_bus::Rgb;

pub struct UserIndication2<BusType: ISmartLedBus> {
    smart_led: BusType,
}

impl<BusType: ISmartLedBus> UserIndication2<BusType> {
    pub fn new(smart_led: BusType) -> Self {
        Self { smart_led }
    }
}

impl<BusType: ISmartLedBus> IUi for UserIndication2<BusType> {
    async fn set(&mut self) {
        let color = [
            Rgb {
                red: 255,
                green: 255,
                blue: 255,
            },
            Rgb {
                red: 0,
                green: 255,
                blue: 255,
            },
            Rgb {
                red: 255,
                green: 255,
                blue: 0,
            },
            Rgb {
                red: 0,
                green: 255,
                blue: 0,
            },
        ];

        self.smart_led.write(&color).await;
    }

    async fn reset(&mut self) {
        let off = [Rgb {
            red: 0,
            green: 0,
            blue: 0,
        }];

        self.smart_led.write(&off).await;
    }
}
