#[cfg(feature = "csharp")]
mod csharp_ffi;

#[cfg(feature = "csharp")]
pub use csharp_ffi::CSharpReader;
