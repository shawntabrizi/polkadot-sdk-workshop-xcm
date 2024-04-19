pub use sandbox::*;

#[cfg(feature = "start")]
mod sandbox {
    pub type Barrier = ();
}

#[cfg(feature = "example")]
mod sandbox {
    use frame_support::traits::Everything;
    use xcm_builder::AllowUnpaidExecutionFrom;

    pub type Barrier = AllowUnpaidExecutionFrom<Everything>;
}
