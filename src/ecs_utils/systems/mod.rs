pub mod physics_systems;
pub mod rendering_systems;

// todo: how do I avoid this function taking idx as an argument?
macro_rules! zip_filter_unwrap {
    ($ray: expr ; $reftype: tt) => {
        zip_filter_unwrap!($ray ; $reftype ; 0)
    };
    ($ray: expr ; $reftype: tt ; $idx: tt) => {
        $ray.into_iter().filter_map(|v| v.$reftype())
    };
    ($($rays: expr ; $reftypes: tt ; $idxs: tt),+) => {
        itertools::izip!($($rays),+)
            .filter(|v| $(v.$idxs.is_some())&&+)
            .map(|v| ($(v.$idxs.$reftypes().unwrap()),+))
    };
}

pub(crate) use zip_filter_unwrap;
