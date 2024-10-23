use scrypto::prelude::*;

#[blueprint]
mod flash_proof {
    enable_method_auth! {
        methods {
            withdraw_nft => restrict_to: [OWNER];
            withdraw_fees => restrict_to: [OWNER];
            update_fee => restrict_to: [OWNER];
            update_stop_timestamp => restrict_to: [OWNER];
            get_nft_proof => PUBLIC;
        }
    }

    struct FlashProof {
        owner_resource: ResourceAddress,
        nft_vault: NonFungibleVault,
        nft_resource: ResourceAddress,
        nflid: NonFungibleLocalId,
        fee_resource: ResourceAddress,
        fee_amount: Decimal,
        fee_vault: Vault,
        stop_timestamp: Instant,
    }

    impl FlashProof {
        pub fn instantiate(
            nft: NonFungibleBucket,
            fee_resource: ResourceAddress,
            fee_amount: Decimal,
            stop_timestamp: Instant,
        ) -> (Global<FlashProof>, FungibleBucket) {
            let (address_reservation, component_address) = Runtime::allocate_component_address(FlashProof::blueprint_id());

            assert!(nft.amount() == Decimal::ONE, "Must supply exactly 1 NFT!");
            assert!(fee_resource.is_fungible(), "Fee resource must be fungible");
            assert!(
                fee_amount > Decimal::ZERO,
                "Fee amount must be higher than 0"
            );

            // Get the resource address and NonFungibleLocalId
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
                        "nft" => nft_id, locked;
                        "component" => GlobalAddress::from(component_address), locked;
                    }
                ))
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1);

            // Instantiate the component and make the supplied owner resource address the owner
            let component = Self {
                owner_resource: owner_badge.resource_address(),
                nft_vault: NonFungibleVault::with_bucket(nft),
                nft_resource,
                nflid,
                fee_resource,
                fee_amount,
                fee_vault: Vault::new(fee_resource),
                stop_timestamp,
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

        // Withdraws funds from the vault
        pub fn withdraw_fees(&mut self) -> FungibleBucket {
            assert!(
                self.fee_vault.amount() > Decimal::ZERO,
                "Nothing to withdraw"
            );

            self.fee_vault.take_all().as_fungible()
        }

        // Updates the fee
        pub fn update_fee(&mut self, amount: Decimal) {
            assert!(amount > Decimal::ZERO, "Fee must be higher than 0");

            self.fee_amount = amount;
        }

        // Updates the stop timestamp
        pub fn update_stop_timestamp(&mut self, timestamp: Instant) {
            assert!(
                timestamp > Clock::current_time_rounded_to_seconds(),
                "Timestamp must be greater than the current time"
            );

            self.stop_timestamp = timestamp;
        }

        // Generates a proof of the NFT and returns it with any
        // remainder of the payment
        pub fn get_nft_proof(&mut self, mut payment: Bucket) -> (NonFungibleProof, Bucket) {
            assert!(
                self.nft_vault.amount() == Decimal::ONE,
                "This component is no longer active."
            );
            assert!(
                payment.resource_address() == self.fee_resource,
                "Did not pay with correct resource!"
            );
            assert!(payment.amount() >= self.fee_amount, "Did not pay enough!");
            assert!(
                Clock::current_time_rounded_to_seconds() < self.stop_timestamp,
                "You can no longer get a proof of this NFT."
            );

            self.fee_vault.put(payment.take(self.fee_amount));

            let mut nflid_set = IndexSet::new();
            nflid_set.insert(self.nflid.clone());
            let proof = self.nft_vault.create_proof_of_non_fungibles(
                &nflid_set
            );

            (proof, payment)
        }
    }
}
