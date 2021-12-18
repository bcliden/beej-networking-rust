use libc::*;
use std::ffi::{CStr, CString};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::{mem, ptr::null};

fn main() {
    let mut hints: addrinfo = unsafe { mem::zeroed() };
    hints.ai_family = AF_UNSPEC;
    hints.ai_socktype = SOCK_STREAM;

    let host: String = std::env::args().skip(1).take(1).collect();
    if host.is_empty() {
        eprintln!("usage: showip hostname");
        return;
    }
    let s = CString::new(host).expect("Didn't work :(");

    let mut result: *mut addrinfo = unsafe { mem::zeroed() };
    let status: i32 = unsafe {
        getaddrinfo(
            s.as_ptr(),
            null(),
            &hints as *const addrinfo,
            &mut result as *mut *mut addrinfo,
        )
    };

    if status != 0 {
        let string = unsafe { CStr::from_ptr(gai_strerror(status)) };
        eprintln!("Uh oh, getaddrinfo failed:\n\t{:?}", string);
        return;
    }

    println!("IP addresses for {:?}", s);

    let mut p = result;
    unsafe {
        while !p.is_null() {
            if (*p).ai_family == AF_INET
            /* is IPv4 */
            {
                let ipv4 = (*p).ai_addr as *mut sockaddr_in;
                let byteorder = u32::from_be((*ipv4).sin_addr.s_addr);
                let ip = Ipv4Addr::from(byteorder);
                println!("\tIPv4: {}", ip);
            } else
            /* is IPv6 */
            {
                let ipv6 = (*p).ai_addr as *mut sockaddr_in6;
                let byteorder = u128::from_be_bytes((*ipv6).sin6_addr.s6_addr);
                let ip = Ipv6Addr::from(byteorder);
                println!("\tIPv6: {}", ip);
            }
            p = (*p).ai_next as *mut addrinfo;
        }
    }
}
