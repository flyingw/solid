use crate::{ffi, DB};
use core::ops::Deref;
use libc::size_t;
use std::marker::PhantomData;
use std::slice;

pub struct WideColumn <'a> {
  name: &'a str,
  value: &'a str,
}

pub struct WideColumns<'a> {
    ptr: *mut ffi::rocksdb_widecolumns_t,
    db: PhantomData<&'a DB>,
}

// unsafe impl Send for WideColumns<'_> {}
// unsafe impl Sync for WideColumns<'_> {}
// impl AsRef<[u8]> for WideColumns<'_> {
// fn as_ref(&self) -> &[u8] { self }
// }

impl Deref for WideColumns<'_> {
  type Target = [u8];

  fn deref(&self) -> &[u8] {
    unsafe {
      let mut len: size_t = 0;
      let val = ffi::rocksdb_widecolumns_value(self.ptr, &mut len) as *mut u8;
      slice::from_raw_parts(val, len)
    }
  }
}

impl Drop for WideColumns<'_> {
    fn drop(&mut self) {
        unsafe {
            ffi::rocksdb_widecolumns_destroy(self.ptr);
        }
    }
}

impl WideColumns<'_> {
    pub(crate) unsafe fn from_c(ptr: *mut ffi::rocksdb_widecolumns_t) -> Self {
        Self { ptr, db: PhantomData, }
    }

    pub fn name(&self) -> &[u8] {
         unsafe {
            let mut len: size_t = 0;
            let val = ffi::rocksdb_widecolumns_name(self.ptr, &mut len) as *mut u8;
            slice::from_raw_parts(val, len)
        }
    }
    pub fn value(&self) -> &[u8] {
         unsafe {
            let mut len: size_t = 0;
            let val = ffi::rocksdb_widecolumns_value(self.ptr, &mut len) as *mut u8;
            slice::from_raw_parts(val, len)
        }
    }
}
