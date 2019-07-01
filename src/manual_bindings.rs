#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum xs_perm_type {
    NONE = 0,
    READ = 1,
    WRITE = 2,
    ENOENT_OK = 4,
    OWNER = 8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xs_permissions {
    pub id: u32,
    pub perms: xs_perm_type,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xs_handle {
    opaque: [u8; 0],
}

#[repr(transparent)]
pub struct xs_transaction(u32);

extern "C" {
    pub fn xs_daemon_rootdir() -> Option<&'static i8>;

    pub fn xs_daemon_rundir() -> Option<&'static i8>;

    pub fn xs_daemon_socket() -> Option<&'static i8>;

    pub fn xs_daemon_socket_ro() -> Option<&'static i8>;

    pub fn xs_domain_dev() -> Option<&'static i8>;

    pub fn xs_daemon_tdb() -> Option<&'static i8>;

    pub fn xs_write_all(fd: i32, data: Option<&u8>, len: u32) -> bool;

    pub fn xs_strings_to_perms(
        perms: Option<&mut xs_permissions>,
        num: u32,
        strings: Option<&u8>,
    ) -> bool;

    pub fn xs_perm_to_string(perm: &xs_permissions, buffer: Option<&u8>, buf_len: usize) -> bool;

    pub fn xs_count_strings(strings: Option<&u8>, len: u32) -> u32;

    pub fn xs_open<'a>(flags: u32) -> Option<&'a xs_handle>;

    pub fn xs_close(xsh: &xs_handle);

    pub fn xs_daemon_open<'a>() -> Option<&'a xs_handle>;

    pub fn xs_domain_open<'a>() -> Option<&'a xs_handle>;

    pub fn xs_daemon_open_readonly<'a>() -> Option<&'a xs_handle>;

    pub fn xs_daemon_close<'a>(arg1: &'a xs_handle);

    pub fn xs_daemon_destroy_postfork<'a>(arg1: &xs_handle);

    pub fn xs_directory<'a, 'b>(
        h: &xs_handle,
        t: xs_transaction,
        path: Option<&i8>,
        num: &mut u32,
    ) -> Option<&'a &'b u8>;

    pub fn xs_read<'a>(
        h: &xs_handle,
        t: xs_transaction,
        path: Option<&i8>,
        len: &mut u32,
    ) -> Option<&'a u8>;

    pub fn xs_write(
        h: &xs_handle,
        t: xs_transaction,
        path: Option<&u8>,
        data: &mut u8,
        len: u32,
    ) -> bool;

    pub fn xs_mkdir(h: &xs_handle, t: xs_transaction, path: Option<&u8>) -> bool;

    pub fn xs_rm(h: &xs_handle, t: xs_transaction, path: Option<&u8>) -> bool;

    pub fn xs_get_permissions<'a>(
        h: &xs_handle,
        t: xs_transaction,
        path: Option<&i8>,
        num: &mut u32,
    ) -> Option<&'a xs_permissions>;

    pub fn xs_set_permissions(
        h: &xs_handle,
        t: xs_transaction,
        path: Option<&u8>,
        perms: &xs_permissions,
        num_perms: u32,
    ) -> bool;

    pub fn xs_watch(h: &xs_handle, path: Option<&i8>, token: Option<&i8>) -> bool;

    pub fn xs_fileno(h: &xs_handle) -> i32;

    pub fn xs_check_watch<'a, 'b>(h: &xs_handle) -> Option<&'a &'b i8>;

    pub fn xs_read_watch<'a, 'b>(h: &xs_handle, num: &mut u32) -> Option<&'a &'b i8>;

    pub fn xs_unwatch(h: &xs_handle, path: Option<&i8>, token: Option<&i8>) -> bool;

    pub fn xs_transaction_start(h: &xs_handle) -> xs_transaction;

    pub fn xs_transaction_end(h: &xs_handle, t: xs_transaction, abort: bool) -> bool;

    pub fn xs_introduce_domain(h: &xs_handle, domid: u32, mfn: u32, eventchn: u32) -> bool;

    pub fn xs_set_target(h: &xs_handle, domid: u32, target: u32) -> bool;

    pub fn xs_resume_domain(h: &xs_handle, domid: u32) -> bool;

    pub fn xs_release_domain(h: &xs_handle, domid: u32) -> bool;

    pub fn xs_get_domain_path(h: &xs_handle, domid: u32) -> &'static i8;

    pub fn xs_path_is_subpath(parent: Option<&u8>, child: Option<&u8>) -> bool;

    pub fn xs_is_domain_introduced(h: &xs_handle, domid: u32) -> bool;

    pub fn xs_control_command<'a>(
        h: &xs_handle,
        cmd: Option<&i8>,
        data: Option<&u8>,
        len: u32,
    ) -> Option<&'a i8>;

    pub fn xs_debug_command<'a>(
        h: &xs_handle,
        cmd: Option<&i8>,
        data: Option<&i8>,
        len: u32,
    ) -> Option<&'a i8>;

    pub fn xs_suspend_evtchn_port(domid: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, ffi::CStr};

    #[test]
    fn test_xs_daemon_rootdir() {
        let dir = "foo";
        env::set_var("XENSTORED_ROOTDIR", dir);
        assert_eq!(dir, unsafe {
            CStr::from_ptr(xs_daemon_rootdir().unwrap())
                .to_str()
                .unwrap()
        });
    }

    #[test]
    fn test_xs_daemon_rundir() {
        let dir = "foo";
        env::set_var("XENSTORED_RUNDIR", dir);
        assert_eq!(dir, unsafe {
            CStr::from_ptr(xs_daemon_rundir().unwrap())
                .to_str()
                .unwrap()
        });
    }

    #[test]
    fn test_xs_daemon_socket() {
        let dir = "foo";
        env::set_var("XENSTORED_PATH", dir);
        assert_eq!(dir, unsafe {
            CStr::from_ptr(xs_daemon_socket().unwrap())
                .to_str()
                .unwrap()
        });
    }

    #[test]
    fn test_xs_daemon_socket_ro() {
        let dir = "foo";
        env::set_var("XENSTORED_PATH", dir);
        assert_eq!(format!("{}_{}", dir, "ro"), unsafe {
            CStr::from_ptr(xs_daemon_socket_ro().unwrap())
                .to_str()
                .unwrap()
        });
    }

    #[test]
    fn test_xs_domain_dev() {
        let dir = "foo";
        env::set_var("XENSTORED_PATH", dir);
        assert_eq!(dir, unsafe {
            CStr::from_ptr(xs_domain_dev().unwrap())
                .to_str()
                .unwrap()
        });
    }
}
