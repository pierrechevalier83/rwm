use wlc;

pub const MOD_KEY: wlc::Modifier::Flags = wlc::Modifier::Logo;

pub const TERMINAL: &[&str] = &["st", "-t", "Terminal"];
pub const MENU: &[&str] = &["dmenu_run"];
pub const WEB: &[&str] = &["tabbed", "-c", "-d", "surf", "-z2", "-e"];
