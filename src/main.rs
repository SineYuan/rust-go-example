use std::mem;
use std::ffi::CString;
use std::os::raw::{c_char, c_void};

#[link(name = "awesome")]
extern {
    fn Add(a: i64, b: i64) -> i64;
    fn Cosine(n: f64) -> f64;
    fn Sort(l: GoSlice);
    fn Log(m: GoString) -> i64;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GoSlice {
    pub data: *mut c_void,
    pub len: isize,
    pub cap: isize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GoString {
    pub p: *const c_char,
    pub n: isize,
}

// Note that both of these methods should probably be implementations
// of the `From` trait to allow them to participate in more places.
impl GoSlice {
    fn from_vec(v: Vec<i64>) -> GoSlice {
//        v.shrink_to_fit(); // ensure capacity == size

        let a = GoSlice {
            data: v.as_ptr() as *mut c_void,
            len: v.len() as isize,
            cap: v.capacity() as isize,
        };

        mem::forget(v);

        a
    }

    fn into_vec(self) -> Vec<i64> {
        unsafe { Vec::from_raw_parts(self.data as *mut i64, self.len as usize, self.cap as usize) }
    }
}

fn main() {
    println!("Using awesome lib from Rust");

    //Call Add() - passing integer params, interger result
    println!("awesome.Add(12,99) = {}", unsafe { Add(12, 99) });

    //Call Cosine() - passing float param, float returned
    println!("awesome.Cosine(1) = {}", unsafe { Cosine(1.0) });

    //Call Sort() - passing an array pointer
    let vec: Vec<i64> = vec!(77, 12, 5, 99, 28, 23);

    let slice = GoSlice::from_vec(vec);

    unsafe { Sort(slice); }

    let sv = slice.into_vec();

    println!("awesome.Sort(77,12,5,99,28,23): = {:?}", sv);

    //Call Log() - passing string value
    let c_to_print = CString::new("Hello from C!").unwrap();
    let msg = GoString { p: c_to_print.as_ptr(), n: 13 };

    let cnt = unsafe { Log(msg) };
    println!("awesome.Log(msg) result: {:?}", cnt);
}