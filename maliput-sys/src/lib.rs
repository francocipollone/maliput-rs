
autocxx::include_cpp! {
    #include "maliput/math/saturate.h"

    generate!("maliput::math::saturate")
    safety!(unsafe)
}

pub use ffi::*;
