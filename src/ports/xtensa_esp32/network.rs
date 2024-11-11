use crate::ports::xtensa_esp32::hardware_timer::{
    CLOCKS, PERIFERALS_RADIO_CLK, PERIFERALS_RNG, PERIFERALS_WIFI, TIMER00,
};
use esp_hal::rng::Rng;
use esp_wifi::{esp_now::EspNow, initialize, EspWifiInitFor};

pub static mut ESP_NOW: Option<EspNow> = None;

/// Network initialization.
pub fn init_network() {
    unsafe {
        let peripherals_rng = PERIFERALS_RNG.take().expect("RNG peripherals error");
        let peripherals_radio_clk = PERIFERALS_RADIO_CLK
            .take()
            .expect("RADIO_CLK peripherals error");
        let timer00 = TIMER00.take().expect("Network timer error");
        let clocks = CLOCKS.take().expect("Network clocks error");
        let periferals_wifi = PERIFERALS_WIFI.take().expect("WIFI peripherals error");

        let init = initialize(
            EspWifiInitFor::Wifi,
            timer00,
            Rng::new(peripherals_rng),
            peripherals_radio_clk,
            &clocks,
        )
        .unwrap();

        ESP_NOW = Some(esp_wifi::esp_now::EspNow::new(&init, periferals_wifi).unwrap());
    }
}

/// Getting esp-now object for network.
pub fn get_esp_now() -> EspNow<'static> {
    unsafe {
        let esp_now = ESP_NOW.take().expect("Esp-now error");
        return esp_now;
    }
}