pub mod fmt {
    use bevy::window::WindowMode;

    /// Formats `WindowMode` to a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::window::WindowMode;
    /// use kgrs_util::fmt::wm_to_string;
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
