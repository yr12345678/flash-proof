A simple blueprint that instantiates a component that holds an NFT and generates a Proof of it on demand, optionally against payment of a fee, for a limited amount of time.

The Proof can be used during a transaction.

<!-- TOC start (generated with https://github.com/derlin/bitdowntoc) -->

- [Package](#package)
   * [Mainnet](#mainnet)
   * [Stokenet](#stokenet)
- [Types](#types)
   * [FeeInfo](#feeinfo)
- [Methods](#methods)
   * [instantiate](#instantiate)
   * [withdraw_nft](#withdraw_nft)
   * [withdraw_fees](#withdraw_fees)
   * [update_fee](#update_fee)
   * [update_end_timestamp](#update_end_timestamp)
   * [get_nft_proof](#get_nft_proof)
- [Manifest examples](#manifest-examples)
   * [Instantiate a component](#instantiate-a-component)
   * [Update the end timestamp](#update-the-end-timestamp)
   * [Update the fee](#update-the-fee)
   * [Withdraw your NFT](#withdraw-your-nft)
   * [Withdraw your fees](#withdraw-your-fees)
   * [Use Flash Proof in a transaction](#use-flash-proof-in-a-transaction)

<!-- TOC end -->

<!-- TOC --><a name="package"></a>
## Package
<!-- TOC --><a name="mainnet"></a>
### Mainnet
`package_rdx1phcw0993dpezja7crhf982s072z6v8ts2z0h8u4j8z5qcgygprds0t`

<!-- TOC --><a name="stokenet"></a>
### Stokenet
`package_tdx_2_1p40nq4a2f09ztx9x0yn42wcqzxjhrmz2npra7ynfphd8ek92x7qgj0`

<!-- TOC --><a name="types"></a>
## Types
<!-- TOC --><a name="feeinfo"></a>
### FeeInfo
A struct containing info about fees to be paid, with the following fields:
* `resource`: ResourceAddress
* `amount`: Decimal

<!-- TOC --><a name="methods"></a>
## Methods
<!-- TOC --><a name="instantiate"></a>
### instantiate
Instantiates a new FlashProof component. Requiring a fee to be paid for Proof generation is optional. An end time is required however, as unlimited Proof generation can be potentially dangerous if it's forgotten about and circumstances change. You can always update the end timestamp.
<!-- TOC --><a name="input"></a>
#### Input
* `nft`: NonFungibleBucket - The NFT that you wish to make available for Proof generation
* `fee_info`: Option\<FeeInfo\> - Optionally set a fee to be paid
* `end_timestamp`: Instant - When should Proof generation stop

<!-- TOC --><a name="output"></a>
#### Output
* The component
* An owner badge

<!-- TOC --><a name="withdraw_nft"></a>
### withdraw_nft
Withdraw your NFT from the component. This effectively disables the component.

* This method is permissioned, it requires a Proof of the owner badge present.
* This method will panic if the NFT is no longer present.
<!-- TOC --><a name="input-1"></a>
#### Input
None

<!-- TOC --><a name="output-1"></a>
#### Output
* The deposited NFT

<!-- TOC --><a name="withdraw_fees"></a>
### withdraw_fees
Withdraw the earned fees from the component.

* This method is permissioned, it requires a Proof of the owner badge present.
* This method will panic if no fee is set.
<!-- TOC --><a name="input-2"></a>
#### Input
None

<!-- TOC --><a name="output-2"></a>
#### Output
* Withdrawn fees

<!-- TOC --><a name="update_fee"></a>
### update_fee
Update the required fee. You can only update the amount. It is also possible to set it to 0, to effectively make it free of charge, but it would still require the user to send in a Bucket.

* This method is permissioned, it requires a Proof of the owner badge present.
* This method will panic if no fee is set.
<!-- TOC --><a name="input-3"></a>
#### Input
* amount: Decimal - The new fee amount.

<!-- TOC --><a name="output-3"></a>
#### Output
None

<!-- TOC --><a name="update_end_timestamp"></a>
### update_end_timestamp
Updates the end timestamp of the Proof generation. After this timestamp, proofs can no longer be generated, unless of course you update the timestamp again.

* This method is permissioned, it requires a Proof of the owner badge present.
* This method will panic if the new timestamp is before the current time.
<!-- TOC --><a name="input-4"></a>
#### Input
* new_timestamp: Instant

<!-- TOC --><a name="output-4"></a>
#### Output
None

<!-- TOC --><a name="get_nft_proof"></a>
### get_nft_proof
Generates a Proof for the NFT stored in the component and returns that with any remainder of the payment (if provided). The Proof ends up in the auth zone.

* This method will panic if:
    * The NFT is no longer in the component
    * The current timestamp is after the end timestamp
    * A payment is required, but was not provided
    * A payment was provided with the wrong resource
    * A payment was provided with the wrong amount
<!-- TOC --><a name="input-5"></a>
#### Input
* `payment`: Bucket

<!-- TOC --><a name="output-5"></a>
#### Output
* The Proof of the NFT
* An Option: either a remainder of the payment or None (if no payment was provided)

<!-- TOC --><a name="manifest-examples"></a>
## Manifest examples
<!-- TOC --><a name="instantiate-a-component"></a>
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

<!-- TOC --><a name="update-the-end-timestamp"></a>
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

<!-- TOC --><a name="update-the-fee"></a>
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

<!-- TOC --><a name="withdraw-your-nft"></a>
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

<!-- TOC --><a name="withdraw-your-fees"></a>
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

<!-- TOC --><a name="use-flash-proof-in-a-transaction"></a>
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
  Proof("my_flash_proof") # Only required if the method explicitly wants the Proof as an input
;

CALL_METHOD
  Address("YOUR_ACCOUNT")
  "deposit_batch"
  Expression("ENTIRE_WORKTOP")
;
```