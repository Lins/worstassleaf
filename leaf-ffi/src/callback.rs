use std::ffi::c_void;
use std::fmt::{Debug, Formatter};
use std::mem::size_of;
use leaf::callback::Callback as Inner;
use std::option::Option;
use std::os::raw::{c_float, c_ulonglong};
use std::ptr::null;
use libc::free;

type ReportTrafficFunc = Option<extern "C" fn(
    tx_rate: c_float,
    rx_rate: c_float,
    tx_total: c_ulonglong,
    rx_total: c_ulonglong,
) -> ()>;

#[repr(C)]
pub struct Callback {
    pub report_traffic: ReportTrafficFunc,
    // Option<extern "C" fn(
    //     tx_rate: c_float,
    //     rx_rate: c_float,
    //     tx_total: c_ulonglong,
    //     rx_total: c_ulonglong,
    // ) -> ()>,
}

#[no_mangle]
pub extern "C" fn create_callback(
    report_traffic: ReportTrafficFunc,
    // Option<extern "C" fn(
    //     tx_rate: c_float,
    //     rx_rate: c_float,
    //     tx_total: c_ulonglong,
    //     rx_total: c_ulonglong,
    // ) -> ()>,
) -> *const Callback {
    unsafe {
        let p = libc::malloc(size_of::<Callback>()) as *mut Callback;
        (*p).report_traffic = report_traffic;
        p
    }
}

#[no_mangle]
pub extern "C" fn destroy_callback(cb: *const Callback) {
    unsafe { free(cb as *mut c_void) }
}

pub(crate) struct FfiCallback {
    inner: *const Callback,
}

impl FfiCallback {
    pub fn new(inner: *const Callback) -> FfiCallback {
        return FfiCallback {
            inner
        };
    }
}

impl Debug for FfiCallback {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

unsafe impl Send for FfiCallback {}

unsafe impl Sync for FfiCallback {}

impl Inner for FfiCallback {
    fn report_traffic(self: &Self, tx_rate: f32, rx_rate: f32, rx_total: u64, tx_total: u64) {
        unsafe {
            let f = (*self.inner).report_traffic.unwrap();
            f(tx_rate as c_float, rx_rate as c_float, rx_total as c_ulonglong, tx_total as c_ulonglong);
        }
    }
}