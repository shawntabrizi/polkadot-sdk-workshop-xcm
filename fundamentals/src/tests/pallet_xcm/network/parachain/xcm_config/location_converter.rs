use super::{AccountId, RelayNetwork};
use core::marker::PhantomData;
use xcm::latest::prelude::*;
use xcm_builder::{
	AccountId32Aliases, DescribeAllTerminal, DescribeFamily, DescribeLocation, HashedDescription,
};
use xcm_executor::traits::ConvertLocation;

/// A location converter which says that the owner of public key on any parachain can access the
/// same public key on any other parachain.
pub struct PublicKeyOwnership<AccountId, Describe>(PhantomData<(AccountId, Describe)>);
impl<AccountId: From<[u8; 32]> + Clone + std::fmt::Debug, Describe: DescribeLocation>
	ConvertLocation<AccountId> for PublicKeyOwnership<AccountId, Describe>
{
	fn convert_location(location: &Location) -> Option<AccountId> {
		Some(match location.unpack() {
			(_, [Parachain(_), AccountId32 { id, .. }]) => (*id).into(),
			_ => return None,
		})
	}
}

type LocationToAccountId = (
	// TODO: Check on this :)
	PublicKeyOwnership<AccountId, DescribeFamily<DescribeAllTerminal>>,
	HashedDescription<AccountId, DescribeFamily<DescribeAllTerminal>>,
	AccountId32Aliases<RelayNetwork, AccountId>,
);

pub type LocationConverter = LocationToAccountId;
