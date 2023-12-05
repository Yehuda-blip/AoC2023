#[cfg(windows)]
pub(crate) const DOUBLE_LINE_ENDING: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
pub(crate) const DOUBLE_LINE_ENDING: &'static str = "\n\n";