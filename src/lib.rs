use scrypto::prelude::*;
use types::FeeInfo;

pub mod types;

#[blueprint]
mod flash_proof {
    enable_method_auth! {
        methods {
            withdraw_nft => restrict_to: [OWNER];
            withdraw_fees => restrict_to: [OWNER];
            update_fee => restrict_to: [OWNER];
            update_end_timestamp => restrict_to: [OWNER];
            get_nft_proof => PUBLIC;
        }
    }

    struct FlashProof {
        owner_resource: ResourceAddress,
        nft_vault: NonFungibleVault,
        nft_id: NonFungibleGlobalId,
        fee_info: Option<FeeInfo>,
        fee_vault: Option<Vault>,
        end_timestamp: Instant,
    }

    impl FlashProof {
        pub fn instantiate(
            nft: NonFungibleBucket,
            fee_info: Option<FeeInfo>,
            end_timestamp: Instant,
        ) -> (Global<FlashProof>, FungibleBucket) {
            // Get an address reservation which we'll use in the description of the owner resource
            let (address_reservation, component_address) = Runtime::allocate_component_address(FlashProof::blueprint_id());

            // If a fee is asked, make sure it's a fungible and the amount is higher than 0
            let mut vault = None;
            if let Some(ref fee_info) = fee_info {
                assert!(fee_info.resource.is_fungible(), "Fee resource must be fungible");
                assert!(
                    fee_info.amount > Decimal::ZERO,
                    "Fee amount must be higher than 0"
                );

                // Create a Vault to store the fee payments in
                vault = Some(Vault::new(fee_info.resource));
            };

            // Get the resource address and NonFungibleLocalId
            assert!(nft.amount() == Decimal::ONE, "Must supply exactly 1 NFT!");
            let nft_resource = nft.resource_address();
            let nflid = nft.non_fungible_local_id();

            // NFT id
            let nft_id = NonFungibleGlobalId::new(
                nft_resource,
                nflid.clone()
            );

            // Create an owner badge
            let owner_badge = ResourceBuilder::new_fungible(OwnerRole::None)
                .metadata(metadata!(
                    init {
                        "symbol" => "FLASHOWN", locked;
                        "name" => "Flash Proof component owner", locked;
                        "description" => "The owner badge for a Flash Proof component. Can be used to update state on the component, withdraw your NFT and claim fees.", locked;
                        "nft" => nft_id.clone(), locked;
                        "component" => GlobalAddress::from(component_address), locked;
                    }
                ))
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1);

            // Instantiate the component and make the supplied owner resource address the owner
            let component = Self {
                owner_resource: owner_badge.resource_address(),
                nft_vault: NonFungibleVault::with_bucket(nft),
                nft_id,
                fee_info,
                fee_vault: vault,
                end_timestamp,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(rule!(require(owner_badge.resource_address()))))
            .with_address(address_reservation)
            .globalize();

            (component, owner_badge)
        }

        // The owner withdraws the NFT. This stops the component from working.
        pub fn withdraw_nft(&mut self) -> NonFungibleBucket {
            assert!(self.nft_vault.amount() > dec!(0), "Nothing to withdraw");

            self.nft_vault.take_all()
        }

        // Withdraws funds from the vault, if a fee is asked.
        pub fn withdraw_fees(&mut self) -> FungibleBucket {
            if self.fee_vault.is_some() {
                assert!(
                    self.fee_vault.as_mut().unwrap().amount() > Decimal::ZERO,
                    "Nothing to withdraw"
                );
    
                self.fee_vault.as_mut().unwrap().take_all().as_fungible()
            } else {
                panic!("This component does not ask for a fee!");
            }
        }

        // Updates the fee
        pub fn update_fee(&mut self, amount: Decimal) {
            if self.fee_vault.is_some() {
                self.fee_info.as_mut().unwrap().amount = amount;
            } else {
                panic!("This component does not ask for a fee!");
            }
        }

        // Updates the end timestamp
        pub fn update_end_timestamp(&mut self, new_timestamp: Instant) {
            // Cannot set a timestamp in the past
            assert!(
                new_timestamp > Clock::current_time_rounded_to_seconds(),
                "Timestamp must be greater than the current time"
            );

            self.end_timestamp = new_timestamp;
        }

        // Generates a proof of the NFT and returns it with any
        // remainder of the payment, if any payment was provided.
        pub fn get_nft_proof(&mut self, mut payment: Option<Bucket>) -> (NonFungibleProof, Option<Bucket>) {
            assert!(
                self.nft_vault.amount() == Decimal::ONE,
                "This component is no longer active."
            );
            assert!(
                Clock::current_time_rounded_to_seconds() < self.end_timestamp,
                "You can no longer get a proof of this NFT."
            );

            // Generate the proof and return it with any remainder from the payment
            let mut nflid_set = IndexSet::new();
            nflid_set.insert(self.nft_id.local_id().clone());
            let proof = self.nft_vault.create_proof_of_non_fungibles(
                &nflid_set
            );            

            // If a payment is required
            if let Some(ref fee_info) = self.fee_info {
                // Make sure a payment was provided
                assert!(payment.is_some(), "No payment was provided");
                // Make sure it was the correct resource
                assert!(
                    payment.as_ref().unwrap().resource_address() == fee_info.resource,
                    "Did not pay with correct resource!"
                );
                // Make sure it was the correct amount
                assert!(payment.as_mut().unwrap().amount() >= fee_info.amount, "Did not pay enough!");
    
                // Take the payment
                self.fee_vault.as_mut().unwrap().put(payment.as_mut().unwrap().take(fee_info.amount));
            }

            // Return proof and either a payment remainder or None
            (proof, payment)
        }
    }
}
