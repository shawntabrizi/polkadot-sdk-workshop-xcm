frame_benchmarking::define_benchmarks!(
	// Only benchmark the following pallets
	[frame_system, SystemBench::<Runtime>]
	[cumulus_pallet_parachain_system, ParachainSystem]
	[pallet_timestamp, Timestamp]
	[pallet_balances, Balances]
	[pallet_sudo, Sudo]
	[pallet_collator_selection, CollatorSelection]
	[pallet_session, SessionBench::<Runtime>]
	[cumulus_pallet_xcmp_queue, XcmpQueue]
	[pallet_message_queue, MessageQueue]
);
