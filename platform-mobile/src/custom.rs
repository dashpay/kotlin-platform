use dashcore::hashes::Hash;
use dashcore::secp256k1::ThirtyTwoByteHash;

#[allow(non_camel_case_types)]
#[repr(C)]
#[ferment_macro::register(dashcore::blockdata::transaction::OutPoint)]
#[derive(Clone)]
pub struct OutPoint {
    pub txid: *mut [u8; 32],
    pub vout: u32,
}
impl ferment_interfaces::FFIConversion<dashcore::blockdata::transaction::OutPoint> for OutPoint {
    unsafe fn ffi_from_const(ffi: *const Self) -> dashcore::blockdata::transaction::OutPoint {
        let ffi = &*ffi;
        dashcore::blockdata::transaction::OutPoint::new(dashcore::hash_types::Txid::from_slice(&*ffi.txid).expect("err"), ffi.vout)
    }
    unsafe fn ffi_to_const(obj: dashcore::blockdata::transaction::OutPoint) -> *const Self {
        ferment_interfaces::boxed(OutPoint { txid: ferment_interfaces::boxed(obj.txid.to_raw_hash().into_32()), vout: obj.vout })
    }
}

impl Drop for OutPoint {
    fn drop(&mut self) {
        unsafe {
            ferment_interfaces::unbox_any(self.txid);
        }
    }
}

// #[ferment_macro::export]
// pub fn get_outpoint(instant_lock: dashcore::blockdata::transaction::OutPoint) -> dashcore::blockdata::transaction::OutPoint {
//     instant_lock.clone()
// }

#[allow(non_camel_case_types)]
#[ferment_macro::register(dashcore::ephemerealdata::instant_lock::InstantLock)]
#[derive(Clone)]
#[repr(C)]
pub struct InstantLock {
    pub raw: *mut dashcore::ephemerealdata::instant_lock::InstantLock,
}
impl ferment_interfaces::FFIConversion<dashcore::ephemerealdata::instant_lock::InstantLock> for InstantLock {
    unsafe fn ffi_from_const(ffi: *const Self) -> dashcore::ephemerealdata::instant_lock::InstantLock {
        let ffi = &*ffi;
        let raw = &*ffi.raw;
        raw.clone()
    }
    unsafe fn ffi_to_const(obj: dashcore::ephemerealdata::instant_lock::InstantLock) -> *const Self {
        ferment_interfaces::boxed(Self { raw: ferment_interfaces::boxed(obj) })
    }
}

impl Drop for InstantLock {
    fn drop(&mut self) {
        unsafe {
            ferment_interfaces::unbox_any(self.raw);
        }
    }
}

// #[ferment_macro::export]
// pub fn get_instantlock(instant_lock: dashcore::ephemerealdata::instant_lock::InstantLock) -> dashcore::ephemerealdata::instant_lock::InstantLock {
//     instant_lock.clone()
// }

#[allow(non_camel_case_types)]
#[ferment_macro::register(dashcore::blockdata::transaction::Transaction)]
#[derive(Clone)]
#[repr(C)]
pub struct Transaction {
    pub raw: *mut dashcore::blockdata::transaction::Transaction,
}
impl ferment_interfaces::FFIConversion<dashcore::blockdata::transaction::Transaction> for Transaction {
    unsafe fn ffi_from_const(ffi: *const Self) -> dashcore::blockdata::transaction::Transaction {
        let ffi = &*ffi;
        let raw = &*ffi.raw;
        raw.clone()
    }
    unsafe fn ffi_to_const(obj: dashcore::blockdata::transaction::Transaction) -> *const Self {
        ferment_interfaces::boxed(Self { raw: ferment_interfaces::boxed(obj) })
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        unsafe {
            ferment_interfaces::unbox_any(self.raw);
        }
    }
}

// #[allow(non_camel_case_types)]
// #[ferment_macro::register(dashcore::consensus::encode::Error)]
// pub struct dashcore_consensus_encode_Error {
//     pub raw: Box<dashcore::consensus::encode::Error>,
// }
// impl ferment_interfaces::FFIConversion<dashcore::consensus::encode::Error> for dashcore_consensus_encode_Error {
//     unsafe fn ffi_from_const(ffi: *const Self) -> dashcore::consensus::encode::Error {
//         *(*ffi).raw
//     }
//     unsafe fn ffi_to_const(obj: dashcore::consensus::encode::Error) -> *const Self {
//         ferment_interfaces::boxed(Self { raw: Box::new(obj) })
//     }
// }