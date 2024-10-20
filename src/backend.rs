use crate::protocal::read;
use crate::sys;
use crate::ui::AppWindow;
use alloc::boxed::Box;
use alloc::sync::Weak;

static mut APP_WEAK: Weak<slint::Weak<AppWindow>> = Weak::new();

unsafe extern "C" fn backend(argc: i32, argv: *mut *mut i8) -> i32 {
    let result = read([0; 5]);
    let people = u32::from_le_bytes(result[0..4].try_into().unwrap());
    let humidity = u32::from_le_bytes(result[4..8].try_into().unwrap());
    let temperature = u32::from_le_bytes(result[8..12].try_into().unwrap());

    let ui = APP_WEAK.upgrade().unwrap();
    ui.upgrade_in_event_loop(move |ui| {
        ui.set_people(people as i32);
        ui.set_humidity(humidity as i32);
        ui.set_temperature(temperature as i32);
    })
    .unwrap();

    return 0;
}
pub fn update_value_singleton(weak: slint::Weak<AppWindow>) {
    unsafe {
        let ptr: *mut slint::Weak<AppWindow> = Box::into_raw(Box::new(weak));
        APP_WEAK.clone_from(&Weak::from_raw(ptr));
    }
    let task = unsafe {
        sys::task_create(
            c"backend".as_ptr(),
            0,
            2048,
            Some(backend),
            core::ptr::null(),
        )
    };
}
