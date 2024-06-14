# Withdraw Assets

## Withdraw Assets

```rust
WithdrawAsset(assets) => {
	let origin = self.origin_ref().ok_or(XcmError::BadOrigin)?;
	Config::TransactionalProcessor::process(|| {
		// Take `assets` from the origin account (on-chain)...
		for asset in assets.inner() {
			Config::AssetTransactor::withdraw_asset(
				asset,
				origin,
				Some(&self.context),
			)?;
		}
		Ok(())
	})
	.and_then(|_| {
		// ...and place into holding.
		self.holding.subsume_assets(assets.into());
		Ok(())
	})
},
```

## Burn Assets

```rust
BurnAsset(assets) => {
	self.holding.saturating_take(assets.into());
	Ok(())
},
```
