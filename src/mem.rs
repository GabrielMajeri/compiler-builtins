#[allow(warnings)]
#[cfg(target_pointer_width = "16")]
type c_int = i16;
#[allow(warnings)]
#[cfg(not(target_pointer_width = "16"))]
type c_int = i32;

#[cfg_attr(all(feature = "mem", not(feature = "mangled-names")), no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut u8,
                                src: *const u8,
                                n: usize)
                                -> *mut u8 {
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let mut i = 0;
        while i < n {
            *dest.offset(i as isize) = *src.offset(i as isize);
            i += 1;
        }
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        asm!("rep movsb" : : "{edi}"(dest), "{esi}"(src), "{ecx}"(n) : "memory");
    }
    dest
}

#[cfg_attr(all(feature = "mem", not(feature = "mangled-names")), no_mangle)]
pub unsafe extern "C" fn memmove(dest: *mut u8,
                                 src: *const u8,
                                 n: usize)
                                 -> *mut u8 {
    if src < dest as *const u8 {
        // copy from end
        let mut i = n;
        while i != 0 {
            i -= 1;
            *dest.offset(i as isize) = *src.offset(i as isize);
        }
    } else {
        // copy from beginning
        let mut i = 0;
        while i < n {
            *dest.offset(i as isize) = *src.offset(i as isize);
            i += 1;
        }
    }
    dest
}

#[cfg_attr(all(feature = "mem", not(feature = "mangled-names")), no_mangle)]
pub unsafe extern "C" fn memset(s: *mut u8, c: c_int, n: usize) -> *mut u8 {
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let mut i = 0;
        while i < n {
            *s.offset(i as isize) = c as u8;
            i += 1;
        }
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        asm!("rep stosb" : : "{edi}"(s), "{al}"(c as u8), "{ecx}"(n) : "memory");
    }
    s
}

#[cfg_attr(all(feature = "mem", not(feature = "mangled-names")), no_mangle)]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.offset(i as isize);
        let b = *s2.offset(i as isize);
        if a != b {
            return a as i32 - b as i32;
        }
        i += 1;
    }
    0
}
