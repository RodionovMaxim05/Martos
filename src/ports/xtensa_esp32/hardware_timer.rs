use core::time::Duration;
use esp_hal::timer::timg::{Timer, Timer0, TimerGroup};
use esp_hal::{
    clock::ClockControl, clock::Clocks, peripherals::*, prelude::*, system::SystemControl,
};

// TODO: initialize peripherals in separate mod
pub static mut TIMER00: Option<Timer<Timer0<TIMG0>, esp_hal::Blocking>> = None;
pub static mut CLOCKS: Option<Clocks> = None;
pub static mut PERIFERALS_RNG: Option<RNG> = None;
pub static mut PERIFERALS_RADIO_CLK: Option<RADIO_CLK> = None;
pub static mut PERIFERALS_WIFI: Option<WIFI> = None;

/// Esp32 hardware timer setup.
pub fn setup_hardware_timer() {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);

    let timer00 = timer_group0.timer0;
    let _ = timer00.load_value(500u64.millis());
    timer00.start();
    timer00.listen();

    unsafe {
        TIMER00 = Some(timer00);
        PERIFERALS_RNG = Some(peripherals.RNG);
        PERIFERALS_RADIO_CLK = Some(peripherals.RADIO_CLK);
        CLOCKS = Some(clocks);
        PERIFERALS_WIFI = Some(peripherals.WIFI);
    }
}

/// Esp32 start harware timer.
pub fn start_hardware_timer() {}

/// Esp32 change operating mode of hardware timer.
pub fn set_reload_mode(auto_reload: bool) {}

/// Esp32 change the period of hardware timer.
pub fn change_period_timer(period: Duration) {}

/// Esp32 getting counter value of hardware timer.
pub fn get_time() -> Duration {
    unsafe {
        let timer00 = TIMER00.take().expect("Timer error");
        let tick_counter = timer00.now();
        TIMER00 = Some(timer00);
        Duration::from_micros(tick_counter.ticks())
    }
}
