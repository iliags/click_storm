use click_storm::settings::cursor_position::CursorPosition;
use click_storm::settings::{app_settings::AppSettings, repeat_type::RepeatType};
use click_storm::worker::worker_thread;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cs_hal::input::{mouse_button::MouseButton, mouse_click::MouseClickType};
use std::sync::{atomic::AtomicBool, Arc};

pub fn criterion_benchmark(c: &mut Criterion) {
    let is_running: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));

    let mut settings: AppSettings = AppSettings::new();
    *settings.interval_milliseconds_mut() = 16;
    *settings.click_type_mut() = MouseClickType::Single;
    *settings.mouse_button_mut() = MouseButton::Left;
    *settings.repeat_type_mut() = RepeatType::Repeat(10);
    *settings.cursor_position_type_mut() = CursorPosition::FixedLocation(500, 500);

    let settings = settings;

    c.bench_function("worker", |b| {
        b.iter(|| worker_thread(black_box(settings.clone()), black_box(is_running.clone())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
