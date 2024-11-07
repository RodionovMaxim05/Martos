use core::time::Duration;
use esp_hal::timer::timg::{Timer, Timer0, TimerGroup};
use esp_hal::{clock::ClockControl, peripherals::*, prelude::*, system::SystemControl};

static mut TIMER00: Option<Timer<Timer0<TIMG0>, esp_hal::Blocking>> = None;

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
    }
}

/// Esp32 start harware timer.
pub fn start_hardware_timer() {}

/// Esp32 change the period of a timer.
pub fn change_period_timer(period: Duration) {}

/// Esp32 getting counter value.
pub fn get_time() -> Duration {
    unsafe {
        let timer00 = TIMER00.take().expect("Timer error");
        let tick_counter = timer00.now();
        TIMER00 = Some(timer00);
        Duration::from_micros(tick_counter.ticks())
    }
}
