/// Adds a new warning with the specified level and message
pub fn add_warning(level: WarningLevel, message: &str) {
    let warning_manager = get_global_warning_manager();
    warning_manager.add_warning(level, message);
}

/// Retrieves all current warnings
pub fn get_warnings() -> Vec<(WarningLevel, String)> {
    let warning_manager = get_global_warning_manager();
    warning_manager.get_warnings()
}

/// Clears all warnings
pub fn clear_warnings() {
    let warning_manager = get_global_warning_manager();
    warning_manager.clear_warnings();
}

/// Gets the global warning manager instance
fn get_global_warning_manager() -> &'static WarningManager {
    use std::sync::Once;
    static mut INSTANCE: Option<WarningManager> = None;
    static INIT: Once = Once::new();

    unsafe {
        INIT.call_once(|| {
            INSTANCE = Some(WarningManager::new());
        });
        INSTANCE.as_ref().unwrap()
    }
}
