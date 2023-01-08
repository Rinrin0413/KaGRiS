use bevy::{prelude::*, time::FixedTimestep};

/// Returns a SystemSet that runs the system 60 times per second.
///
/// # Examples
///
/// ```
/// app.add_system_set(fixed_update(foo_system));
/// ```
pub fn fixed_update<T>(system: impl IntoSystemDescriptor<T>) -> SystemSet {
    let step = 0.01666666666666666666666666666666667; // 1/60
    SystemSet::new()
        .with_run_criteria(FixedTimestep::step(step))
        .with_system(system)
}

pub mod fmt {
    use bevy::window::WindowMode;

    /// Formats `WindowMode` to a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::window::WindowMode;
    /// use kgrs_util::function::fmt::wm_to_string;
    ///
    /// assert_eq!(wm_to_string(WindowMode::Windowed), "Windowed");
    /// assert_eq!(wm_to_string(WindowMode::BorderlessFullscreen), "Borderless Fullscreen");
    /// assert_eq!(wm_to_string(WindowMode::SizedFullscreen), "Sized Fullscreen");
    /// assert_eq!(wm_to_string(WindowMode::Fullscreen), "Fullscreen");
    /// ```
    pub fn wm_to_string(mode: WindowMode) -> String {
        match mode {
            WindowMode::Windowed => "Windowed",
            WindowMode::BorderlessFullscreen => "Borderless Fullscreen",
            WindowMode::SizedFullscreen => "Sized Fullscreen",
            WindowMode::Fullscreen => "Fullscreen",
        }
        .to_string()
    }
}
