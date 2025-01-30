//! Session Objects

use rexegy_sys::XCSession;

#[derive(Default, Debug)]
pub struct Session(Option<XCSession>);