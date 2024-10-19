use crate::slint_support;
use alloc::boxed::Box;
use anyhow::{anyhow, Result};

slint::include_modules!();

pub fn run_ui() -> Result<()> {
    let platform = Box::new(slint_support::MIXRT::new()?);
    slint::platform::set_platform(platform).map_err(|_| anyhow!("Cannot set platform"))?;
    let ui = AppWindow::new().map_err(|_| anyhow!("Cannot create UI"))?;
    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });

    ui.run().map_err(|_| anyhow!("Cannot run UI"))?;
    Ok(())
}
