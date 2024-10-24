A simple blueprint that instantiates a component that holds an NFT and generates a Proof of it on demand, optionally against payment of a fee, for a limited amount of time.

The Proof can be used during a transaction.

## Package
### Mainnet
`package_rdx1phcw0993dpezja7crhf982s072z6v8ts2z0h8u4j8z5qcgygprds0t`

### Stokenet
`package_tdx_2_1p40nq4a2f09ztx9x0yn42wcqzxjhrmz2npra7ynfphd8ek92x7qgj0`

## Types
### FeeInfo
A struct containing info about fees to be paid, with the following fields:
* `resource`: ResourceAddress
* `amount`: Decimal

## Methods
### instantiate
Instantiates a new FlashProof component. Requiring a fee to be paid for Proof generation is optional. An end time is required however, as unlimited Proof generation can be potentially dangerous if it's forgotten about and circumstances change. You can always update the end timestamp.
#### Input
* `nft`: NonFungibleBucket - The NFT that you wish to make available for Proof generation
* `fee_info`: Option\<FeeInfo\> - Optionally set a fee to be paid
* `end_timestamp`: Instant - When should Proof generation stop

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
    * The current timestamp is after the end timestamp
    * A payment is required, but was not provided
    * A payment was provided with the wrong resource
    * A payment was provided with the wrong amount
#### Input
* `payment`: Bucket

#### Output
* The Proof of the NFT
* An Option: either a remainder of the payment or None (if no payment was provided)

## Manifest examples
### Instantiate a component
```
CALL_METHOD
  Address("YOUR_ACCOUNT")
  "withdraw_non_fungibles"
  Address("NFT_RESOURCE")
  Array<NonFungibleLocalId>(
    NonFungibleLocalId("NFT_ID")
  )
;

TAKE_ALL_FROM_WORKTOP
  Address("NFT_RESOURCE")
  Bucket("nft")
;

CALL_FUNCTION
  Address("package_rdx1phcw0993dpezja7crhf982s072z6v8ts2z0h8u4j8z5qcgygprds0t") # Mainnet
  "FlashProof"
  "instantiate"
  Bucket("nft")
  # Apply a fee of 420 $EARLY. Replace with Enum<0u8>() or None to instantiate without a fee requirement.
  Enum<1u8>(
    Tuple(
      Address("resource_rdx1t5xv44c0u99z096q00mv74emwmxwjw26m98lwlzq6ddlpe9f5cuc7s"),
      Decimal("420")
    )
  )
  1729756098i64
;

CALL_METHOD
  Address("YOUR_ACCOUNT")
  "deposit_batch"
  Expression("ENTIRE_WORKTOP")
;
```

### Update the end timestamp
```
CALL_METHOD
  Address("YOUR_ACCOUNT")
  "create_proof_of_amount"
  Address("YOUR_OWNER_BADGE_RESOURCE")
  Decimal("1")
;

CALL_METHOD
  Address("YOUR_FLASH_PROOF_COMPONENT")
  "update_end_timestamp"
  1731000886i64
;
```

### Update the fee
```
CALL_METHOD
  Address("YOUR_ACCOUNT")
  "create_proof_of_amount"
  Address("YOUR_OWNER_BADGE_RESOURCE")
  Decimal("1")
;

CALL_METHOD
  Address("YOUR_FLASH_PROOF_COMPONENT")
  "update_fee"
  Decimal("69")
;
```

### Withdraw your NFT
```
CALL_METHOD
  Address("YOUR_ACCOUNT")
  "create_proof_of_amount"
  Address("YOUR_OWNER_BADGE_RESOURCE")
  Decimal("1")
;

CALL_METHOD
  Address("YOUR_FLASH_PROOF_COMPONENT")
  "withdraw_nft"
;

CALL_METHOD
  Address("YOUR_ACCOUNT")
  "deposit_batch"
  Expression("ENTIRE_WORKTOP")
;
```

### Withdraw your fees
```
CALL_METHOD
  Address("YOUR_ACCOUNT")
  "create_proof_of_amount"
  Address("YOUR_OWNER_BADGE_RESOURCE")
  Decimal("1")
;

CALL_METHOD
  Address("YOUR_FLASH_PROOF_COMPONENT")
  "withdraw_fees"
;

CALL_METHOD
  Address("YOUR_ACCOUNT")
  "deposit_batch"
  Expression("ENTIRE_WORKTOP")
;
```

### Use Flash Proof in a transaction
```
# Scenario assumes a fee is required
CALL_METHOD
  Address("YOUR_ACCOUNT")
  "withdraw"
  Address("FEE_RESOURCE")
  Decimal("FEE_AMOUNT")
;

TAKE_ALL_FROM_WORKTOP
  Address("FEE_RESOURCE")
  Bucket("fee_payment")
;

# Get the Proof from the Flash Proof component
CALL_METHOD
  Address("FLASH_PROOF_COMPONENT")
  "get_nft_proof"
  Enum<1u8>(Bucket("fee_payment"))
;

# Proof ended up in the auth zone from which it can be used if the
# method you're calling doesn't require it explicitly as an input.
# Use the POP_FROM_AUTH_ZONE below to create a named Proof that
# you can use as input for a method if required.
POP_FROM_AUTH_ZONE
  Proof("my_flash_proof")
;

# Call the method using the Proof as an input.
CALL_METHOD
  Address("SOME_COMPONENT")
  "some_method"
  Proof("my_flash_rpoof") # Only required if the method explicitly wants the Proof as an input
;

CALL_METHOD
  Address("YOUR_ACCOUNT")
  "deposit_batch"
  Expression("ENTIRE_WORKTOP")
;
```