/* automatically generated by rust-bindgen 0.71.1 */

pub type ISTDUINT32 = ::core::ffi::c_uint;
pub type ISTDINT32 = ::core::ffi::c_int;
pub type IINT32 = ISTDINT32;
pub type IUINT32 = ISTDUINT32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IQUEUEHEAD {
    pub next: *mut IQUEUEHEAD,
    pub prev: *mut IQUEUEHEAD,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IQUEUEHEAD"][::core::mem::size_of::<IQUEUEHEAD>() - 16usize];
    ["Alignment of IQUEUEHEAD"][::core::mem::align_of::<IQUEUEHEAD>() - 8usize];
    ["Offset of field: IQUEUEHEAD::next"][::core::mem::offset_of!(IQUEUEHEAD, next) - 0usize];
    ["Offset of field: IQUEUEHEAD::prev"][::core::mem::offset_of!(IQUEUEHEAD, prev) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IKCPCB {
    pub conv: IUINT32,
    pub mtu: IUINT32,
    pub mss: IUINT32,
    pub state: IUINT32,
    pub snd_una: IUINT32,
    pub snd_nxt: IUINT32,
    pub rcv_nxt: IUINT32,
    pub ts_recent: IUINT32,
    pub ts_lastack: IUINT32,
    pub ssthresh: IUINT32,
    pub rx_rttval: IINT32,
    pub rx_srtt: IINT32,
    pub rx_rto: IINT32,
    pub rx_minrto: IINT32,
    pub snd_wnd: IUINT32,
    pub rcv_wnd: IUINT32,
    pub rmt_wnd: IUINT32,
    pub cwnd: IUINT32,
    pub probe: IUINT32,
    pub current: IUINT32,
    pub interval: IUINT32,
    pub ts_flush: IUINT32,
    pub xmit: IUINT32,
    pub nrcv_buf: IUINT32,
    pub nsnd_buf: IUINT32,
    pub nrcv_que: IUINT32,
    pub nsnd_que: IUINT32,
    pub nodelay: IUINT32,
    pub updated: IUINT32,
    pub ts_probe: IUINT32,
    pub probe_wait: IUINT32,
    pub dead_link: IUINT32,
    pub incr: IUINT32,
    pub snd_queue: IQUEUEHEAD,
    pub rcv_queue: IQUEUEHEAD,
    pub snd_buf: IQUEUEHEAD,
    pub rcv_buf: IQUEUEHEAD,
    pub acklist: *mut IUINT32,
    pub ackcount: IUINT32,
    pub ackblock: IUINT32,
    pub user: *mut ::core::ffi::c_void,
    pub buffer: *mut ::core::ffi::c_char,
    pub fastresend: ::core::ffi::c_int,
    pub fastlimit: ::core::ffi::c_int,
    pub nocwnd: ::core::ffi::c_int,
    pub stream: ::core::ffi::c_int,
    pub logmask: ::core::ffi::c_int,
    pub output: ::core::option::Option<
        unsafe extern "C" fn(
            buf: *const ::core::ffi::c_char,
            len: ::core::ffi::c_int,
            kcp: *mut IKCPCB,
            user: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub writelog: ::core::option::Option<
        unsafe extern "C" fn(
            log: *const ::core::ffi::c_char,
            kcp: *mut IKCPCB,
            user: *mut ::core::ffi::c_void,
        ),
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IKCPCB"][::core::mem::size_of::<IKCPCB>() - 272usize];
    ["Alignment of IKCPCB"][::core::mem::align_of::<IKCPCB>() - 8usize];
    ["Offset of field: IKCPCB::conv"][::core::mem::offset_of!(IKCPCB, conv) - 0usize];
    ["Offset of field: IKCPCB::mtu"][::core::mem::offset_of!(IKCPCB, mtu) - 4usize];
    ["Offset of field: IKCPCB::mss"][::core::mem::offset_of!(IKCPCB, mss) - 8usize];
    ["Offset of field: IKCPCB::state"][::core::mem::offset_of!(IKCPCB, state) - 12usize];
    ["Offset of field: IKCPCB::snd_una"][::core::mem::offset_of!(IKCPCB, snd_una) - 16usize];
    ["Offset of field: IKCPCB::snd_nxt"][::core::mem::offset_of!(IKCPCB, snd_nxt) - 20usize];
    ["Offset of field: IKCPCB::rcv_nxt"][::core::mem::offset_of!(IKCPCB, rcv_nxt) - 24usize];
    ["Offset of field: IKCPCB::ts_recent"][::core::mem::offset_of!(IKCPCB, ts_recent) - 28usize];
    ["Offset of field: IKCPCB::ts_lastack"][::core::mem::offset_of!(IKCPCB, ts_lastack) - 32usize];
    ["Offset of field: IKCPCB::ssthresh"][::core::mem::offset_of!(IKCPCB, ssthresh) - 36usize];
    ["Offset of field: IKCPCB::rx_rttval"][::core::mem::offset_of!(IKCPCB, rx_rttval) - 40usize];
    ["Offset of field: IKCPCB::rx_srtt"][::core::mem::offset_of!(IKCPCB, rx_srtt) - 44usize];
    ["Offset of field: IKCPCB::rx_rto"][::core::mem::offset_of!(IKCPCB, rx_rto) - 48usize];
    ["Offset of field: IKCPCB::rx_minrto"][::core::mem::offset_of!(IKCPCB, rx_minrto) - 52usize];
    ["Offset of field: IKCPCB::snd_wnd"][::core::mem::offset_of!(IKCPCB, snd_wnd) - 56usize];
    ["Offset of field: IKCPCB::rcv_wnd"][::core::mem::offset_of!(IKCPCB, rcv_wnd) - 60usize];
    ["Offset of field: IKCPCB::rmt_wnd"][::core::mem::offset_of!(IKCPCB, rmt_wnd) - 64usize];
    ["Offset of field: IKCPCB::cwnd"][::core::mem::offset_of!(IKCPCB, cwnd) - 68usize];
    ["Offset of field: IKCPCB::probe"][::core::mem::offset_of!(IKCPCB, probe) - 72usize];
    ["Offset of field: IKCPCB::current"][::core::mem::offset_of!(IKCPCB, current) - 76usize];
    ["Offset of field: IKCPCB::interval"][::core::mem::offset_of!(IKCPCB, interval) - 80usize];
    ["Offset of field: IKCPCB::ts_flush"][::core::mem::offset_of!(IKCPCB, ts_flush) - 84usize];
    ["Offset of field: IKCPCB::xmit"][::core::mem::offset_of!(IKCPCB, xmit) - 88usize];
    ["Offset of field: IKCPCB::nrcv_buf"][::core::mem::offset_of!(IKCPCB, nrcv_buf) - 92usize];
    ["Offset of field: IKCPCB::nsnd_buf"][::core::mem::offset_of!(IKCPCB, nsnd_buf) - 96usize];
    ["Offset of field: IKCPCB::nrcv_que"][::core::mem::offset_of!(IKCPCB, nrcv_que) - 100usize];
    ["Offset of field: IKCPCB::nsnd_que"][::core::mem::offset_of!(IKCPCB, nsnd_que) - 104usize];
    ["Offset of field: IKCPCB::nodelay"][::core::mem::offset_of!(IKCPCB, nodelay) - 108usize];
    ["Offset of field: IKCPCB::updated"][::core::mem::offset_of!(IKCPCB, updated) - 112usize];
    ["Offset of field: IKCPCB::ts_probe"][::core::mem::offset_of!(IKCPCB, ts_probe) - 116usize];
    ["Offset of field: IKCPCB::probe_wait"][::core::mem::offset_of!(IKCPCB, probe_wait) - 120usize];
    ["Offset of field: IKCPCB::dead_link"][::core::mem::offset_of!(IKCPCB, dead_link) - 124usize];
    ["Offset of field: IKCPCB::incr"][::core::mem::offset_of!(IKCPCB, incr) - 128usize];
    ["Offset of field: IKCPCB::snd_queue"][::core::mem::offset_of!(IKCPCB, snd_queue) - 136usize];
    ["Offset of field: IKCPCB::rcv_queue"][::core::mem::offset_of!(IKCPCB, rcv_queue) - 152usize];
    ["Offset of field: IKCPCB::snd_buf"][::core::mem::offset_of!(IKCPCB, snd_buf) - 168usize];
    ["Offset of field: IKCPCB::rcv_buf"][::core::mem::offset_of!(IKCPCB, rcv_buf) - 184usize];
    ["Offset of field: IKCPCB::acklist"][::core::mem::offset_of!(IKCPCB, acklist) - 200usize];
    ["Offset of field: IKCPCB::ackcount"][::core::mem::offset_of!(IKCPCB, ackcount) - 208usize];
    ["Offset of field: IKCPCB::ackblock"][::core::mem::offset_of!(IKCPCB, ackblock) - 212usize];
    ["Offset of field: IKCPCB::user"][::core::mem::offset_of!(IKCPCB, user) - 216usize];
    ["Offset of field: IKCPCB::buffer"][::core::mem::offset_of!(IKCPCB, buffer) - 224usize];
    ["Offset of field: IKCPCB::fastresend"][::core::mem::offset_of!(IKCPCB, fastresend) - 232usize];
    ["Offset of field: IKCPCB::fastlimit"][::core::mem::offset_of!(IKCPCB, fastlimit) - 236usize];
    ["Offset of field: IKCPCB::nocwnd"][::core::mem::offset_of!(IKCPCB, nocwnd) - 240usize];
    ["Offset of field: IKCPCB::stream"][::core::mem::offset_of!(IKCPCB, stream) - 244usize];
    ["Offset of field: IKCPCB::logmask"][::core::mem::offset_of!(IKCPCB, logmask) - 248usize];
    ["Offset of field: IKCPCB::output"][::core::mem::offset_of!(IKCPCB, output) - 256usize];
    ["Offset of field: IKCPCB::writelog"][::core::mem::offset_of!(IKCPCB, writelog) - 264usize];
};
pub type ikcpcb = IKCPCB;
unsafe extern "C" {
    pub fn ikcp_create(conv: IUINT32, user: *mut ::core::ffi::c_void) -> *mut ikcpcb;
}
unsafe extern "C" {
    pub fn ikcp_release(kcp: *mut ikcpcb);
}
unsafe extern "C" {
    pub fn ikcp_setoutput(
        kcp: *mut ikcpcb,
        output: ::core::option::Option<
            unsafe extern "C" fn(
                buf: *const ::core::ffi::c_char,
                len: ::core::ffi::c_int,
                kcp: *mut ikcpcb,
                user: *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    );
}
unsafe extern "C" {
    pub fn ikcp_recv(
        kcp: *mut ikcpcb,
        buffer: *mut ::core::ffi::c_char,
        len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn ikcp_send(
        kcp: *mut ikcpcb,
        buffer: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn ikcp_update(kcp: *mut ikcpcb, current: IUINT32);
}
unsafe extern "C" {
    pub fn ikcp_check(kcp: *const ikcpcb, current: IUINT32) -> IUINT32;
}
unsafe extern "C" {
    pub fn ikcp_input(
        kcp: *mut ikcpcb,
        data: *const ::core::ffi::c_char,
        size: ::core::ffi::c_long,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn ikcp_flush(kcp: *mut ikcpcb);
}
unsafe extern "C" {
    pub fn ikcp_peeksize(kcp: *const ikcpcb) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn ikcp_setmtu(kcp: *mut ikcpcb, mtu: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn ikcp_wndsize(
        kcp: *mut ikcpcb,
        sndwnd: ::core::ffi::c_int,
        rcvwnd: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn ikcp_waitsnd(kcp: *const ikcpcb) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn ikcp_nodelay(
        kcp: *mut ikcpcb,
        nodelay: ::core::ffi::c_int,
        interval: ::core::ffi::c_int,
        resend: ::core::ffi::c_int,
        nc: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    pub fn ikcp_log(
        kcp: *mut ikcpcb,
        mask: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
}
unsafe extern "C" {
    pub fn ikcp_allocator(
        new_malloc: ::core::option::Option<
            unsafe extern "C" fn(arg1: usize) -> *mut ::core::ffi::c_void,
        >,
        new_free: ::core::option::Option<unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void)>,
    );
}
unsafe extern "C" {
    pub fn ikcp_getconv(ptr: *const ::core::ffi::c_void) -> IUINT32;
}
