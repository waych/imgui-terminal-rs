#[cxx::bridge(namespace = "terminal")]
mod ffi {
    // Any shared structs, whose fields will be visible to both languages.

    // extern "Rust" {
    //     // Zero or more opaque types which both lagnauges can pass around but only Rust can see the
    //     // fields.

    //     // type MultiBuf;

    //     // Functions implemented in rust.
    //     // fn next_chunk(buf: &mut MultiBuf) -> &[u8];
    // }

    unsafe extern "C++" {
        // One or more headers with the matching C++ declarations.  Our code generators don't read
        // it but it gets #include'd and used in static assertions to ensure our picture of the FFI
        // boundary is accurate.
        include!("imgui-terminal-rs.h");

        // Zero or more opaque types which both languages can pass around but only C++ can see the
        // fields.
        type Terminal;

        // Functions implemented in C++.
        fn new_terminal() -> UniquePtr<Terminal>;
        fn reset(self: Pin<&mut Terminal>);
        fn draw_term(terminal: Pin<&mut Terminal>, str_id: String);
        fn write_term(terminal: Pin<&mut Terminal>, bytes: &str);
        fn read_term(terminal: Pin<&mut Terminal>, bytes: &mut [u8]) -> i32;
    }
}

pub struct Terminal {
    inner: cxx::UniquePtr<ffi::Terminal>,
}
impl Terminal {
    pub fn new() -> Option<Self> {
        let inner = ffi::new_terminal();
        if inner.is_null() {
            return None;
        }
        Some(Self { inner: ffi::new_terminal() })
    }
    pub fn as_inner(&mut self) -> std::pin::Pin<&mut ffi::Terminal> {
        self.inner.pin_mut()
    }
    /// Reset the terminal.
    pub fn reset(&mut self) {
        self.as_inner().reset()
    }
    /// Draw the terminal window.
    ///
    /// open is an out-parameter indicating whether the window is (still) open.
    pub fn draw(&mut self, str_id: &imgui::ImStr) {
        ffi::draw_term(self.as_inner(), str_id.to_string())
    }
    /// Write characters to the terminal.
    /// TODO: Is it correct to send rust utf8 slices like this?
    pub fn write(&mut self, bytes: &str) {
        ffi::write_term(self.as_inner(), bytes)
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> i32 {
        ffi::read_term(self.as_inner(), buffer)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
