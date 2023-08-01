/// https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/buttons#value
#[allow(dead_code)]
#[repr(u16)]
pub enum MouseButtons {
    Primary = 1,
    Secondary = 2,
    Auxiliary = 4,
    BrowserBack = 8,
    BrowserForward = 16,
}
