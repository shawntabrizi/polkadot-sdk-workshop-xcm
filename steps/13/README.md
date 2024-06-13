# Transfer Asset

```rust
TransferAsset { assets, beneficiary } => {
	Config::TransactionalProcessor::process(|| {
		// Take `assets` from the origin account (on-chain) and place into dest account.
		let origin = self.origin_ref().ok_or(XcmError::BadOrigin)?;
		for asset in assets.inner() {
			Config::AssetTransactor::transfer_asset(
				&asset,
				origin,
				&beneficiary,
				&self.context,
			)?;
		}
		Ok(())
	})
},
```
