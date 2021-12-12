#[cfg_attr(target_os = "linux", path = "unix.rs")]
#[cfg_attr(windows, path = "windows.rs")]
pub mod platform;

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        assert_eq!(super::platform::platform(), 0);
    }
}
