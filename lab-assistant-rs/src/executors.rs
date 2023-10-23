use crate::{staratlas, Client, Pubkey, Signer};

use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};

use std::ops::Deref;

#[derive(borsh::BorshSerialize)]
struct FleetStateHandler;

pub fn exec_stop_mining<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    // payer: &Keypair,
    // game_info: &SagePlayerProfileGameState,
    fleet: &(Pubkey, [Pubkey; 3]),
    planet: &(Pubkey, [Pubkey; 1]),
) -> anyhow::Result<()> {
    let program = client.program(staratlas::sage::SAGE_PROGRAM_ID)?;

    // ix: fleetStateHandler
    // let ix = Instruction::new_with_borsh(
    //     staratlas::sage::SAGE_PROGRAM_ID,
    //     &FleetStateHandler,
    //     vec![AccountMeta::new(..., true)]);

    // let request = program.request();
    //     .instruction();

    Ok(())
}

// async function execStopMining(fleet, sageResource, sageResourceAcctInfo, mineItem, resourceToken) {
//     return new Promise(async resolve => {
//         let planet = sageResourceAcctInfo.location;
//         let targetX = fleet.destCoord.split(',')[0].trim();
//         let targetY = fleet.destCoord.split(',')[1].trim();
//         let starbase = await getStarbaseFromCoords(targetX, targetY);
//         let starbasePlayer = await getStarbasePlayer(userProfileAcct,starbase.publicKey);

//         ...snip...

//         await solanaConnection.getAccountInfo(fleetResourceToken) || await createProgramDerivedAccount(fleetResourceToken, fleet.cargoHold, resourceToken);
//         let foodCargoTypeAcct = cargoTypes.find(item => item.account.mint.toString() == sageGameAcct.account.mints.food);
//         let ammoCargoTypeAcct = cargoTypes.find(item => item.account.mint.toString() == sageGameAcct.account.mints.ammo);
//         let resourceCargoTypeAcct = cargoTypes.find(item => item.account.mint.toString() == resourceToken.toString());
//         let tx1 = { instruction: await sageProgram.methods.fleetStateHandler().accountsStrict({
//             fleet: fleet.publicKey
//         }).remainingAccounts([
//             {
//                 pubkey: userProfileFactionAcct.publicKey,
//                 isSigner: false,
//                 isWritable: false
//             },
//             {
//                 pubkey: fleet.cargoHold,
//                 isSigner: false,
//                 isWritable: true
//             },
//             {
//                 pubkey: fleet.ammoBank,
//                 isSigner: false,
//                 isWritable: true
//             },
//             {
//                 pubkey: mineItem,
//                 isSigner: false,
//                 isWritable: false
//             },
//             {
//                 pubkey: sageResource, //Account5
//                 isSigner: false,
//                 isWritable: true
//             },
//             {
//                 pubkey: planet,
//                 isSigner: false,
//                 isWritable: true
//             },
//             {
//                 pubkey: starbase.publicKey,
//                 isSigner: false,
//                 isWritable: false
//             },
//             {
//                 pubkey: fleetFoodToken, //foodTokenFrom
//                 isSigner: false,
//                 isWritable: true
//             },
//             {
//                 pubkey: fleetAmmoToken, //ammoTokenFrom
//                 isSigner: false,
//                 isWritable: true
//             },
//             {
//                 pubkey: planetResourceToken, //resourceTokenFrom
//                 isSigner: false,
//                 isWritable: true
//             },
//             {
//                 pubkey: fleetResourceToken, //resourceTokenTo
//                 isSigner: false,
//                 isWritable: true
//             },
//             {
//                 pubkey: sageGameAcct.account.mints.food,
//                 isSigner: false,
//                 isWritable: true
//             },
//             {
//                 pubkey: sageGameAcct.account.mints.ammo,
//                 isSigner: false,
//                 isWritable: true
//             },
//             {
//                 pubkey: foodCargoTypeAcct.publicKey,
//                 isSigner: false,
//                 isWritable: false
//             },
//             {
//                 pubkey: ammoCargoTypeAcct.publicKey,
//                 isSigner: false,
//                 isWritable: false
//             },
//             {
//                 pubkey: resourceCargoTypeAcct.publicKey,
//                 isSigner: false,
//                 isWritable: false
//             },
//             {
//                 pubkey: sageGameAcct.account.cargo.statsDefinition,
//                 isSigner: false,
//                 isWritable: false
//             },
//             {
//                 pubkey: sageGameAcct.publicKey,
//                 isSigner: false,
//                 isWritable: false
//             },
//             {
//                 pubkey: new solanaWeb3.PublicKey('Cargo8a1e6NkGyrjy4BQEW4ASGKs9KSyDyUrXMfpJoiH'),
//                 isSigner: false,
//                 isWritable: false
//             },
//             {
//                 pubkey: new solanaWeb3.PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'),
//                 isSigner: false,
//                 isWritable: false
//             },
//         ]).instruction()}

//         let tx2 = { instruction: await sageProgram.methods.stopMiningAsteroid({keyIndex: 0}).accountsStrict({
//             gameAccountsFleetAndOwner: {
//                 gameFleetAndOwner: {
//                     fleetAndOwner: {
//                         fleet: fleet.publicKey,
//                         owningProfile: userProfileAcct,
//                         owningProfileFaction: userProfileFactionAcct.publicKey,
//                         key: userPublicKey
//                     },
//                     gameId: sageGameAcct.publicKey
//                 },
//                 gameState: sageGameAcct.account.gameState
//             },
//             resource: sageResource,
//             planet: planet,
//             fuelTank : fleet.fuelTank,
//             cargoType: fuelCargoTypeAcct.publicKey,
//             cargoStatsDefinition: sageGameAcct.account.cargo.statsDefinition,
//             tokenFrom: fleet.fuelToken,
//             tokenMint: sageGameAcct.account.mints.fuel,
//             cargoProgram: new solanaWeb3.PublicKey('Cargo8a1e6NkGyrjy4BQEW4ASGKs9KSyDyUrXMfpJoiH'),
//             tokenProgram: new solanaWeb3.PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'),
//         }).instruction()}
//         let txResult = await txSignAndSend([tx1,tx2]);
//         console.log('---STOP MINE---');
//         console.log(txResult);
//         resolve(txResult);
//     });
// }

// it('stop mining', async function () {
//     // wait for some mining to happen
//     await new Promise((r) => setTimeout(r, 1000));
//     const fleetAccount = await readFromRPCOrError(
//       provider.connection,
//       program,
//       fleet1,
//       Fleet,
//       'confirmed',
//     );
//     const mint = _resourceMintKeys[1];
//     if (!mint) {
//       throw 'mint data does not exist';
//     }
//     const mineItemKey = MineItem.findAddress(
//       program,
//       _game.publicKey(),
//       mint.publicKey,
//     )[0];
//     const resourceKey = Resource.findAddress(
//       program,
//       mineItemKey,
//       _planet.publicKey(),
//     )[0];
//     const resourceTokenFrom = await getAssociatedTokenAddress(
//       mint.publicKey,
//       mineItemKey,
//       true,
//     );
//     const resourceTokenTo = await getAssociatedTokenAddress(
//       mint.publicKey,
//       fleetAccount.data.cargoHold,
//       true,
//     );
//     const fleetFoodToken = await getAssociatedTokenAddress(
//       _food.publicKey,
//       fleetAccount.data.cargoHold,
//       true,
//     );
//     const fleetAmmoToken = await getAssociatedTokenAddress(
//       _ammo.publicKey,
//       fleetAccount.data.ammoBank,
//       true,
//     );
//     const fleetFuelToken = await getAssociatedTokenAddress(
//       _fuel.publicKey,
//       fleetAccount.data.fuelTank,
//       true,
//     );
//     const resourceBefore = await readFromRPCOrError(
//       provider.connection,
//       program,
//       resourceKey,
//       Resource,
//       'confirmed',
//     );
//     const planetBefore = await readFromRPCOrError(
//       provider.connection,
//       program,
//       _planet.publicKey(),
//       Planet,
//       'confirmed',
//     );
//     const resourceTokenBefore = await getAccount(
//       program.provider.connection,
//       resourceTokenTo,
//       'processed',
//     );

//     // unauthorized key cannot stop mining
//     let success = true;
//     try {
//       await buildSendAndCheck(
//         [
//           Fleet.asteroidMiningHandler(
//             program,
//             cargoProgram,
//             ProfileFactionAccount.findAddress(
//               profileFactionProgram,
//               playerProfile.publicKey(),
//             )[0],
//             fleet1,
//             _starbaseKey,
//             mineItemKey,
//             resourceKey,
//             _planet.publicKey(),
//             fleetAccount.data.cargoHold,
//             fleetAccount.data.ammoBank,
//             CargoType.findAddress(
//               cargoProgram,
//               _cargoStatsDefinitionSigner.publicKey(),
//               _food.publicKey,
//               0,
//             )[0],
//             CargoType.findAddress(
//               cargoProgram,
//               _cargoStatsDefinitionSigner.publicKey(),
//               _ammo.publicKey,
//               0,
//             )[0],
//             CargoType.findAddress(
//               cargoProgram,
//               _cargoStatsDefinitionSigner.publicKey(),
//               mint.publicKey,
//               0,
//             )[0],
//             _cargoStatsDefinitionSigner.publicKey(),
//             _gameState,
//             _game.publicKey(),
//             fleetFoodToken,
//             fleetAmmoToken,
//             resourceTokenFrom,
//             resourceTokenTo,
//             _food.publicKey,
//             _ammo.publicKey,
//           ),
//           Fleet.stopMiningAsteroid(
//             program,
//             cargoProgram,
//             _fleetRespawnSigner /** not authorized */,
//             playerProfile.publicKey(),
//             ProfileFactionAccount.findAddress(
//               profileFactionProgram,
//               playerProfile.publicKey(),
//             )[0],
//             fleet1,
//             resourceKey,
//             _planet.publicKey(),
//             fleetAccount.data.fuelTank,
//             CargoType.findAddress(
//               cargoProgram,
//               _cargoStatsDefinitionSigner.publicKey(),
//               _fuel.publicKey,
//               0,
//             )[0],
//             _cargoStatsDefinitionSigner.publicKey(),
//             _gameState,
//             _game.publicKey(),
//             fleetFuelToken,
//             _fuel.publicKey,
//             { keyIndex: CAN_MINE_WITH_FLEET },
//           ),
//         ],
//         walletSigner,
//         provider.connection,
//       );
//     } catch (err) {
//       expect(JSON.stringify(err as Error)).toContain('KeyMismatch');
//       success = false;
//     }
//     expect(success).toBe(false);
//     // can stop mining okay
//     await buildSendAndCheck(
//       [
//         Fleet.asteroidMiningHandler(
//           program,
//           cargoProgram,
//           ProfileFactionAccount.findAddress(
//             profileFactionProgram,
//             playerProfile.publicKey(),
//           )[0],
//           fleet1,
//           _starbaseKey,
//           mineItemKey,
//           resourceKey,
//           _planet.publicKey(),
//           fleetAccount.data.cargoHold,
//           fleetAccount.data.ammoBank,
//           CargoType.findAddress(
//             cargoProgram,
//             _cargoStatsDefinitionSigner.publicKey(),
//             _food.publicKey,
//             0,
//           )[0],
//           CargoType.findAddress(
//             cargoProgram,
//             _cargoStatsDefinitionSigner.publicKey(),
//             _ammo.publicKey,
//             0,
//           )[0],
//           CargoType.findAddress(
//             cargoProgram,
//             _cargoStatsDefinitionSigner.publicKey(),
//             mint.publicKey,
//             0,
//           )[0],
//           _cargoStatsDefinitionSigner.publicKey(),
//           _gameState,
//           _game.publicKey(),
//           fleetFoodToken,
//           fleetAmmoToken,
//           resourceTokenFrom,
//           resourceTokenTo,
//           _food.publicKey,
//           _ammo.publicKey,
//         ),
//         Fleet.stopMiningAsteroid(
//           program,
//           cargoProgram,
//           _fleetMinerSigner,
//           playerProfile.publicKey(),
//           ProfileFactionAccount.findAddress(
//             profileFactionProgram,
//             playerProfile.publicKey(),
//           )[0],
//           fleet1,
//           resourceKey,
//           _planet.publicKey(),
//           fleetAccount.data.fuelTank,
//           CargoType.findAddress(
//             cargoProgram,
//             _cargoStatsDefinitionSigner.publicKey(),
//             _fuel.publicKey,
//             0,
//           )[0],
//           _cargoStatsDefinitionSigner.publicKey(),
//           _gameState,
//           _game.publicKey(),
//           fleetFuelToken,
//           _fuel.publicKey,
//           { keyIndex: CAN_MINE_WITH_FLEET },
//         ),
//       ],
//       walletSigner,
//       provider.connection,
//     );
//     const fleetAccountAfter = await readFromRPCOrError(
//       provider.connection,
//       program,
//       fleet1,
//       Fleet,
//       'confirmed',
//     );
//     const resourceAfter = await readFromRPCOrError(
//       provider.connection,
//       program,
//       resourceKey,
//       Resource,
//       'confirmed',
//     );
//     const planetAfter = await readFromRPCOrError(
//       provider.connection,
//       program,
//       _planet.publicKey(),
//       Planet,
//       'confirmed',
//     );
//     const sectorAcc = await readFromRPCOrError(
//       provider.connection,
//       program,
//       _sectorKey,
//       Sector,
//       'confirmed',
//     );
//     const resourceTokenAfter = await getAccount(
//       program.provider.connection,
//       resourceTokenTo,
//       'processed',
//     );
//     if (fleetAccountAfter.state.Idle == null) {
//       throw 'Fleet not idle';
//     }
//     expect(
//       arrayDeepEquals(
//         fleetAccountAfter.state.Idle?.sector,
//         sectorAcc.data.coordinates,
//         (a, b) => a.eq(b),
//       ),
//     ).toBe(true);
//     expect(
//       resourceAfter.data.numMiners.eq(
//         resourceBefore.data.numMiners.sub(new BN(1)),
//       ),
//     ).toBe(true);
//     expect(
//       planetAfter.data.numMiners.eq(planetBefore.data.numMiners.sub(new BN(1))),
//     ).toBe(true);
//     expect(
//       resourceTokenAfter.delegatedAmount >= resourceTokenBefore.delegatedAmount,
//     ).toBe(true);
//   });