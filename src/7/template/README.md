# Origin

We are going to start simple and implement an instruction with


## Clear Origin

Origin The Consensus System from which a given message has been (directly and immediately) delivered. Specified as a Location.

Expresses the location with whose authority the current programme is running. May be reset to None at will (implying no authority), and may also be set to a strictly interior location at will (implying a strict subset of authority).

When the term Origin is used, it is meant to mean "location whose value is that of the Origin Register". Thus the phrase "controlled by the Origin" is equivalent to "controlled by the location whose value is that of the Origin Register".

Clear the origin.

This may be used by the XCM author to ensure that later instructions cannot command the authority of the origin (e.g. if they are being relayed from an untrusted source, as often the case with `ReserveAssetDeposited`).

```rust
ClearOrigin => {
	self.context.origin = None;
	Ok(())
},
```

## Descend Origin

```rust
DescendOrigin(who) => self
	.context
	.origin
	.as_mut()
	.ok_or(XcmError::BadOrigin)?
	.append_with(who)
	.map_err(|_| XcmError::LocationFull),
```
