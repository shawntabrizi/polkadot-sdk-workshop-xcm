# Deposit Asset

## Deposit Asset

```rust
DepositAsset { assets, beneficiary } => {
	let old_holding = self.holding.clone();
	let result = Config::TransactionalProcessor::process(|| {
		let deposited = self.holding.saturating_take(assets);
		for asset in deposited.into_assets_iter() {
			Config::AssetTransactor::deposit_asset(
				&asset,
				&beneficiary,
				Some(&self.context),
			)?;
		}
		Ok(())
	});
	if Config::TransactionalProcessor::IS_TRANSACTIONAL && result.is_err() {
		self.holding = old_holding;
	}
	result
},
```

## Receive Teleported Asset

```rust
ReceiveTeleportedAsset(assets) => {
	Config::TransactionalProcessor::process(|| {
		let origin = self.origin_ref().ok_or(XcmError::BadOrigin)?;
		// check whether we trust origin to teleport this asset to us via config trait.
		for asset in assets.inner() {
			// We should check that the asset can actually be teleported in (for this to
			// be in error, there would need to be an accounting violation by one of the
			// trusted chains, so it's unlikely, but we don't want to punish a possibly
			// innocent chain/user).
			Config::AssetTransactor::can_check_in(origin, asset, &self.context)?;
			Config::AssetTransactor::check_in(origin, asset, &self.context);
		}
		Ok(())
	})
	.and_then(|_| {
		self.holding.subsume_assets(assets.into());
		Ok(())
	})
},
```
