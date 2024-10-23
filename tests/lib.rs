use scrypto::NonFungibleData;
use scrypto_test::prelude::*;

use flash_proof::flash_proof_test::*;

#[derive(ScryptoSbor, NonFungibleData, Clone, Debug)]
pub struct FakeNFT {
    pub name: String,
}

#[test]
fn can_withdraw_nft() -> Result<(), RuntimeError> {
    // Arrange
    let (mut env, mut component, owner_badge, nft_resource, nflid) =
        create_environment(XRD, dec!(50), 30)?;

    let owner_proof = ProofFactory::create_fungible_proof(
        owner_badge.resource_address(&mut env)?,
        Decimal::ONE,
        Mock,
        &mut env,
    )?;

    LocalAuthZone::push(owner_proof, &mut env)?;

    // Act
    let returned_nft = component.withdraw_nft(&mut env)?;

    // Assert
    assert!(
        returned_nft.0.amount(&mut env)? == Decimal::ONE,
        "Got back another amount than 1 NFT"
    );

    let returned_address = returned_nft.0.resource_address(&mut env)?;
    assert!(
        returned_address == nft_resource,
        "Got back an NFT with a different resource address"
    );

    let returned_nflid = returned_nft
        .0
        .non_fungible_local_ids(&mut env)?
        .first()
        .unwrap()
        .clone();

    assert!(
        returned_nflid == nflid,
        "Got back an NFT with a different NonFungibleLocalId"
    );

    Ok(())
}

#[test]
fn can_get_proof_within_timeframe() -> Result<(), RuntimeError> {
    // Arrange
    let (mut env, mut component, _, nft_resource, nflid) =
        create_environment(XRD, dec!(50), 30)?;

    let payment = BucketFactory::create_fungible_bucket(XRD, dec!(50), Mock, &mut env)?;

    // Act
    let (proof, remainder) = component.get_nft_proof(payment, &mut env)?;

    // Assert
    assert!(
        remainder.amount(&mut env)? == dec!(0),
        "Got back more than expected"
    );

    let proof_address = proof.0.resource_address(&mut env)?;
    let proof_nflid = proof
        .0
        .non_fungible_local_ids(&mut env)?
        .first()
        .unwrap()
        .clone();
    assert!(proof_address == nft_resource, "Got back some weird-ass NFT");
    assert!(proof_nflid == nflid, "Got back some weird-ass NFT");

    Ok(())
}

#[test]
fn can_withdraw_fee() -> Result<(), RuntimeError> {
    // Arrange
    let (mut env, mut component, owner_badge, _, _) =
        create_environment(XRD, dec!(50), 30)?;

    let payment = BucketFactory::create_fungible_bucket(XRD, dec!(50), Mock, &mut env)?;

    let owner_proof = owner_badge.create_proof_of_amount(dec!(1), &mut env)?;
    LocalAuthZone::push(owner_proof, &mut env)?;

    // Act
    let _ = component.get_nft_proof(payment, &mut env)?; // Make payment
    let fee = component.withdraw_fees(&mut env)?; // Withdraw fees

    // Assert
    assert!(
        fee.0.amount(&mut env)? == dec!(50),
        "Got back wrong fee amount"
    );

    Ok(())
}

#[test]
fn can_update_stop_timestamp() -> Result<(), RuntimeError> {
    // Arrange
    let (mut env, mut component, owner_badge, _, _) =
        create_environment(XRD, dec!(50), 30)?;

    let owner_proof = owner_badge.create_proof_of_amount(dec!(1), &mut env)?;
    LocalAuthZone::push(owner_proof, &mut env)?;

    let future_timestamp = env
        .get_current_time()
        .add_days(60)
        .unwrap()
        .add_seconds(1)
        .unwrap();

    // Act
    component.update_stop_timestamp(future_timestamp, &mut env)?;

    // Assert
    let new_stop_timestamp = env
        .with_component_state::<FlashProofState, _, _, _>(component, |state, _env| {
            state.stop_timestamp
        })?;
    
    println!("Future timestamp: {:?}", future_timestamp);
    println!("New timestamp: {:?}", new_stop_timestamp);
    assert!(new_stop_timestamp == future_timestamp, "Timestamp was not properly updated!");

    Ok(())
}

#[test]
fn cannot_get_proof_with_wrong_payment() -> Result<(), RuntimeError> {
    // Arrange
    let (mut env, mut component, _, _, _) =
        create_environment(XRD, dec!(50), 30)?;

    let random_resource= ResourceBuilder::new_fungible(OwnerRole::None)
        .mint_initial_supply(500, &mut env)?;

    let too_little_xrd = BucketFactory::create_fungible_bucket(XRD, dec!(30), Mock, &mut env)?;
    let not_xrd = BucketFactory::create_fungible_bucket(random_resource.resource_address(&mut env)?, dec!(50), Mock, &mut env)?;

    // Act
    let result_too_little_xrd = component.get_nft_proof(too_little_xrd, &mut env);
    let result_not_xrd = component.get_nft_proof(not_xrd, &mut env);

    // Assert
    assert!(
        result_too_little_xrd.is_err(),
        "Did not expect to be able to get proof while paying too little"
    );

    assert!(
        result_not_xrd.is_err(),
        "Did not expect to be able to get proof while not paying with XRD"
    );    

    Ok(())
}

#[test]
fn cannot_get_proof_outside_timeframe() -> Result<(), RuntimeError> {
    // Arrange
    let (mut env, mut component, _, _, _) =
        create_environment(XRD, dec!(50), 30)?;

    let payment = BucketFactory::create_fungible_bucket(XRD, dec!(50), Mock, &mut env)?;

    let future_timestamp = env
        .get_current_time()
        .add_days(30)
        .unwrap()
        .add_seconds(1)
        .unwrap();
    env.set_current_time(future_timestamp);

    // Act
    let result = component.get_nft_proof(payment, &mut env);

    // Assert
    assert!(result.is_err(), "Was able to get proof after timeframe!");

    Ok(())
}

fn create_environment(
    fee_resource: ResourceAddress,
    fee_amount: Decimal,
    days_in_future: i64,
) -> Result<
    (
        TestEnvironment<InMemorySubstateDatabase>,
        FlashProof,
        Bucket,
        ResourceAddress,
        NonFungibleLocalId,
    ),
    RuntimeError,
> {
    let mut env = TestEnvironment::new();
    let package_address =
        PackageFactory::compile_and_publish(this_package!(), &mut env, CompileProfile::Fast)?;

    // Create timestamp
    let timestamp = env.get_current_time().add_days(days_in_future).unwrap();

    // Create fake nft
    let nft = NonFungibleBucket(
        ResourceBuilder::new_ruid_non_fungible(OwnerRole::None).mint_initial_supply(
            vec![FakeNFT {
                name: "My Fake NFT".to_string(),
            }],
            &mut env,
        )?,
    );

    // Get NFT address
    let nft_address = nft.0.resource_address(&mut env)?;

    // Get NonFungibleLocalId
    let nflid = nft
        .0 // Again the .0 needed??
        .non_fungible_local_ids(&mut env)?
        .first()
        .unwrap()
        .clone();

    // Instantiate component with the owner resource address and the NFT
    let (component, owner_badge) = FlashProof::instantiate(
        nft,
        fee_resource,
        fee_amount,
        timestamp,
        package_address,
        &mut env,
    )?;

    Ok((env, component, owner_badge.into(), nft_address, nflid))
}
