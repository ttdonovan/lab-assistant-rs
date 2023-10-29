import { describe, expect, test, beforeAll } from 'bun:test';
import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import bs58 from 'bs58';

import { Fleet } from '@staratlas/sage';

import { SageGameHandler } from '../src/sageGameHandler';
import { SageFleetHandler } from '../src/sageFleetHandler';

let playerPubkey: PublicKey;
let playerProfilePubkey: PublicKey;
let sageGameHandler: SageGameHandler;

// Warning: This function will send transactions to the Solana network (if ENABLED_TX = true)
const sendSageGameTx = async (gameHander: SageGameHandler, tx: any) => {
    const ENABLED_TX = true;

    console.log('--- [tx: start] ---')
    console.log(tx);
    console.log('--- [tx: end] ---')

    if (ENABLED_TX) {
        console.log('--- [rx: start] ---')
        let rx = await sageGameHandler.sendTransaction(tx);
        console.log(rx);
        console.log('--- [rx: start] ---')
    }
}

beforeAll(async () => {
    const rpc_url = Bun.env.SOLANA_RPC_URL || 'http://localhost:8899';

    const connection = new Connection(rpc_url, 'confirmed');

    const secretKey = Bun.env.SOLANA_WALLET_SECRET_KEY;
    if (!secretKey) {
        throw new Error('SOLANA_WALLET_SECRET_KEY environent variable is not set');
    }

    const secretKeyBytes = bs58.decode(secretKey);
    const walletKeypair = Keypair.fromSecretKey(secretKeyBytes);

    playerPubkey = new PublicKey(Bun.env.STAR_ATLAS_PLAYER_PROFILE || walletKeypair);

    sageGameHandler = new SageGameHandler(walletKeypair, connection);

    if (!PublicKey.isOnCurve(sageGameHandler.funder.publicKey().toBytes())) {
        throw 'Funder public key is not on curve';
    }

    await sageGameHandler.ready;
    playerProfilePubkey = await sageGameHandler.getPlayerProfileAddress(playerPubkey);
    await sageGameHandler.loadGame();
})

describe('SAGE Labs (tx)', () => {
    let fleetAccount: Fleet;
    let fleetPubkey: PublicKey;
    let sageFleetHandler: SageFleetHandler;

    beforeAll(async () => {
        sageFleetHandler = new SageFleetHandler(sageGameHandler);
    });

    describe('Fleet Handler - Mining Actions', () => {
        const miningFleetName = 'MINING#1';

        beforeAll(async () => {
            fleetPubkey = sageGameHandler.getFleetAddress(playerProfilePubkey, miningFleetName);
            fleetAccount = await sageFleetHandler.getFleetAccount(fleetPubkey);
            // console.log(fleetAccount);
        });

        test.skip('Start Mining', async () => {
            if (fleetAccount.state.Idle) {
                let ix = await sageFleetHandler.ixStartMining(fleetPubkey, 'hydrogen');
                let tx = await sageGameHandler.buildAndSignTransaction(ix);
                await sendSageGameTx(sageGameHandler, tx);
            }
        });

        test.skip('FleetHandler - Stop Mining', async () => {
            if (fleetAccount.state.MineAsteroid) {
                let ix = await sageFleetHandler.ixStopMining(fleetPubkey);
                let tx = await sageGameHandler.buildAndSignTransaction(ix);
                await sendSageGameTx(sageGameHandler, tx);
            }
        });
    });
})
