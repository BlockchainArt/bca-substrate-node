# Blockchain.art Substrate Node

A [Substrate](https://www.substrate.io/)-based network for the fine art community, made with ❤️ by
[Blockchain.art](https://blockchain.art/).

## BCA Pallet

The runtime of the BCA network is built using the [FRAME](https://substrate.dev/docs/en/knowledgebase/runtime/frame)
system for runtime development, and includes [a custom pallet](pallets/bca/src/lib.rs) that exposes NFT capabilities.
The BCA pallet is built on top of the official
[FRAME Uniques pallet](https://github.com/paritytech/substrate/tree/master/frame/uniques). The API of the BCA pallet is
as follows:

### Create Collection

Create a new collection that represents a single piece of artwork that may have multiple artist proofs and multiple
prints "minted" in the form of NFTs.

#### Parameters

- `metadata [Vec<u8>]`: a reference to offchain metadata about the collection, if a protocol isn't specified it's
  interpreted as IPFS CID (must be unique)
- `edition [{ proofs: u8, prints: u16 }]`: the maximum number of artist proofs and prints that may be minted from this
  collection

#### Events

- `CollectionCreated(collectionOwner, collectionID)`

#### Errors

- `CollectionUnavailable`: a collection with the specified metadata already exists

### Create Print

Mint a new artist proof or print from an existing collection and assign ownership to an account.

#### Parameters

- `collection_id [u32]`: the ID of the collection to mint from
- `proof [bool]`: whether or not to mint an artist proof
- `owner [AccountId]`: the ID of the account to which the proof or print should belong

#### Events

- `PrintCreated(printOwner, collectionID, printID)`

#### Errors

- `CollectionNotFound`: the collection identified by `collection_id` does not exist OR the originator of the request is
  not the collection's owner
- `PrintUnavailable`: the maximum number of artist proofs or prints has already been created

### Transfer Print

Transfer an existing artist proof or print from its owner to another account.

#### Parameters

- `collection_id [u32]`: the ID of the collection to which the artist proof or print belongs
- `print_id [u32]`: the ID of the print to transfer
- `dest [AccountID]`: the ID of the account that will be the new owner of the artist proof or print

#### Events

- `PrintTransferred(newOwner, collectionID, printID)`

#### Errors

- `PrintNotFound`: the artist proof or print identified by `collection_id` & `print_id` does not exist OR the originator
  of the request is not the print's owner

### Genesis Configuration

The BCA pallet exposes the following genesis configuration parameter, which allows users to specify the collections and
artist proofs/prints that should exist in a chain's genesis block:

- `prints [Vec<(T::AccountId, Vec<(Vec<u8>, Edition, Vec<(bool, T::AccountId)>)>)>]`: a list of tuples that map accounts
  to the collections they own; each collection has a list of artist proofs/prints and the accounts to which they belong

## Upstream

This project was forked from the official
[Substrate Developer Hub Node Template](https://github.com/substrate-developer-hub/substrate-node-template).
