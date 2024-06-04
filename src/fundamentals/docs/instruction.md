# Instructions

We are familiar at this point that Cross-Consensus Messages are used to allow different consensus systems to communicate and interoperate with one another.

So now let's take a look at how those XCM messages are composed.

## Composition of an XCM

Let's start by looking at the definition of an XCM:

```rust
pub struct Xcm<Call>(pub Vec<Instruction<Call>>);
```

So an XCM is just a wrapper for a vector of `Instructions`. It is generic over `Call` which is the callable functions exposed by your consensus system. For example, on a Substrate chain, these would be the Pallet Calls.

We will take a high level look at this generic `Call` type and how it is used later.

## XCM Instructions

So XCM messages are just a vector of instructions... but what is an `Instruction`.

At the time of writing, there are nearly 50 different instructions exposed by the XCM format.

At a high level, instructions are specific actions used to manipulate either or both of the underlying blockchain state and the XCM in-memory state.

Let's take a look at just a few examples of instructions to get an idea for what they do:

- `WithdrawAsset`: Moves assets from the ownership of `origin` to the holding registrar.
- `DepositAsset`: Moves assets from the holding registrar to a `beneficiary`.
-

### RFCs

You might be interested to know how new instructions get added to XCM. For this we have the [XCM Format repo](https://github.com/paritytech/xcm-format) and an [RFC process](https://github.com/paritytech/xcm-format/blob/master/proposals/0000-template.md).

We won't go into details about this in this guide, but feel free to browse around there.
