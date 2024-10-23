A simple component that holds an NFT and generates a Proof of it on demand, optionally against payment of a fee, for a limited amount of time.

## Package
### Mainnet

### Stokenet

## Types
### FeeInfo
A struct containing info about fees to be paid, with the following fields:
* resource: ResourceAddress
* amount: Decimal

## Interface
### instantiate
Instantiates a new FlashProof component. Requiring a fee to be paid for Proof generation is optional. An end time is required however, as unlimited Proof generation can be potentially dangerous if it's forgotten about and circumstances change. You can always update the end timestamp.
#### Input
* nft: NonFungibleBucket - The NFT that you wish to make available for Proof generation
* fee_info: Option<FeeInfo> - Optionally set a fee to be paid
* end_timestamp: Instant - When should Proof generation stop

#### Output
* The component
* An owner badge

### withdraw_nft
Withdraw your NFT from the component. This effectively disables the component.

* This method is permissioned, it requires a Proof of the owner badge present.
* This method will panic if the NFT is no longer present.
#### Input
None

#### Output
* The deposited NFT

### withdraw_fees
Withdraw the earned fees from the component.

* This method is permissioned, it requires a Proof of the owner badge present.
* This method will panic if no fee is set.
#### Input
None

#### Output
* Withdrawn fees

### update_fee
Update the required fee. You can only update the amount. It is also possible to set it to 0, to effectively make it free of charge, but it would still require the user to send in a Bucket.

* This method is permissioned, it requires a Proof of the owner badge present.
* This method will panic if no fee is set.
#### Input
* amount: Decimal - The new fee amount.

#### Output
None

### update_end_timestamp
Updates the end timestamp of the Proof generation. After this timestamp, proofs can no longer be generated, unless of course you update the timestamp again.

* This method is permissioned, it requires a Proof of the owner badge present.
* This method will panic if the new timestamp is before the current time.
#### Input
* new_timestamp: Instant

#### Output
None

### get_nft_proof
Generates a Proof for the NFT stored in the component and returns that with any remainder of the payment (if provided). The Proof ends up in the auth zone.

* This method will panic if:
    * The NFT is no longer in the component
    * The curernt timestamp is after the end timestamp
    * A payment is required, but was not provided
    * A payment was provided with the wrong resource
    * A payment was provided with the wrong amount
#### Input
* payment: Bucket

#### Output
* The Proof of the NFT
* An Option: either a remainder of the payment or None (if no payment was provided)