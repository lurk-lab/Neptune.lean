use crate::ffi::{rust_lean_neptune_poseidon, lean_object};
use blstrs::Scalar as Fr;
use cxx::CxxVector;
use ff::PrimeField;
use generic_array::sequence::GenericSequence;
use generic_array::typenum::{U11, U8};
use generic_array::GenericArray;
use neptune::{
  poseidon::{HashMode, Poseidon, PoseidonConstants},
  Arity, Strength,
};

fn poseidon(data: &CxxVector<u8>) -> Vec<u8> {
  let data: Vec<u8> = data.iter().map(|&u| u).collect();
  let constants = PoseidonConstants::new_with_strength(Strength::Standard);
  let mut hasher = Poseidon::<Fr, U11>::new(&constants);
  let scalar = Fr::from(123);
  hasher.input(scalar).unwrap();
  hasher
    .hash_in_mode(HashMode::Correct)
    .to_bytes_le()
    .to_vec()
}

#[no_mangle]
pub unsafe extern "C" fn lean_neptune_poseidon(obj: *mut lean_object) -> *mut lean_object {
  rust_lean_neptune_poseidon(obj)
}

#[cxx::bridge]
mod ffi {
  // Any shared structs, whose fields will be visible to both languages.
  struct Poseidon {
    size: usize,
    tags: Vec<String>,
  }

  extern "Rust" {
    // Zero or more opaque types which both languages can pass around but
    // only Rust can see the fields.
    // type Poseidon<F>;

    // Functions implemented in Rust.
    fn poseidon(data: &CxxVector<u8>) -> Vec<u8>;
  }

  unsafe extern "C++" {
    // One or more headers with the matching C++ declarations. Our code
    // generators don't read it but it gets #include'd and used in static
    // assertions to ensure our picture of the FFI boundary is accurate.
    include!("lean/lean.h");
    include!("lean-neptune-bindings/include/neptune-shim.hpp");
    // Zero or more opaque types which both languages can pass around but
    // only C++ can see the fields.
    type lean_object;

    // Functions implemented in C++.
    pub unsafe fn rust_lean_neptune_poseidon(obj: *mut lean_object) -> *mut lean_object;

    // fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
    // fn put(&self, parts: &mut MultiBuf) -> u64;
    // fn tag(&self, blobid: u64, tag: &str);
    // fn metadata(&self, blobid: u64) -> BlobMetadata;
  }
}
