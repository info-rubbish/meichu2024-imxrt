use crate::backend::update_value_singleton;
use crate::slint_support;
use alloc::boxed::Box;
use anyhow::{anyhow, Result};

slint::include_modules!();

pub fn run_ui() -> Result<()> {
    let platform = Box::new(slint_support::MIXRT::new()?);
    slint::platform::set_platform(platform).map_err(|_| anyhow!("Cannot set platform"))?;
    let ui = AppWindow::new().map_err(|_| anyhow!("Cannot create UI"))?;

    update_value_singleton(ui.as_weak());

    ui.run().map_err(|_| anyhow!("Cannot run UI"))?;
    Ok(())
}
