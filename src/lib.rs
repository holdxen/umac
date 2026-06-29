#[allow(non_camel_case_types)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/umac.rs"));
}

pub trait UMac {
    const KEY_LEN: usize;
    const TAG_LEN: usize;
}

pub struct UMac64 {
    ctx: *mut ffi::umac_ctx,
}

unsafe impl Send for UMac64 {}
unsafe impl Sync for UMac64 {}

impl UMac for UMac64 {
    const KEY_LEN: usize = 16;
    const TAG_LEN: usize = 8;
}

impl Drop for UMac64 {
    fn drop(&mut self) {
        unsafe {
            let ret = ffi::umac_delete(self.ctx);
            assert_eq!(ret, 1);
        }
    }
}

impl UMac64 {
    pub fn new(mut key: [u8; Self::KEY_LEN]) -> Self {
        unsafe {
            let ctx = ffi::umac_new(key.as_mut_ptr() as _);
            Self { ctx }
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        unsafe {
            let ret =
                ffi::umac_update(self.ctx, data.as_ptr() as _, data.len().try_into().unwrap());
            assert_eq!(ret, 1);
        }
    }

    pub fn finalize(self, nonce: [u8; 8]) -> [u8; Self::TAG_LEN] {
        unsafe {
            let mut tag = [0u8; Self::TAG_LEN];
            let ret = ffi::umac_final(self.ctx, tag.as_mut_ptr() as _, nonce.as_ptr() as _);
            assert_eq!(ret, 1);
            tag
        }
    }
}

pub struct UMac128 {
    ctx: *mut ffi::umac_ctx,
}

unsafe impl Send for UMac128 {}
unsafe impl Sync for UMac128 {}

impl UMac for UMac128 {
    const KEY_LEN: usize = 16;

    const TAG_LEN: usize = 16;
}

impl Drop for UMac128 {
    fn drop(&mut self) {
        unsafe {
            let ret = ffi::umac128_delete(self.ctx);
            assert_eq!(ret, 1);
        }
    }
}

impl UMac128 {
    pub fn new(mut key: [u8; Self::KEY_LEN]) -> Self {
        unsafe {
            let ctx = ffi::umac128_new(key.as_mut_ptr() as _);
            Self { ctx }
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        unsafe {
            let ret =
                ffi::umac128_update(self.ctx, data.as_ptr() as _, data.len().try_into().unwrap());
            assert_eq!(ret, 1);
        }
    }

    pub fn finalize(self, nonce: [u8; 8]) -> [u8; Self::TAG_LEN] {
        unsafe {
            let mut tag = [0u8; Self::TAG_LEN];
            let ret = ffi::umac128_final(self.ctx, tag.as_mut_ptr() as _, nonce.as_ptr() as _);
            assert_eq!(ret, 1);
            tag
        }
    }
}
