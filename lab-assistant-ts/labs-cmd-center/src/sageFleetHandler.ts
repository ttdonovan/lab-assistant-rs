import { BN } from '@project-serum/anchor';
import { getAssociatedTokenAddress } from '@solana/spl-token';
import { PublicKey } from '@solana/web3.js';
import { readFromRPCOrError, InstructionReturn, createAssociatedTokenAccountIdempotent } from '@staratlas/data-source';
import {
    Fleet,
    MineItem,
    Planet,
    Resource,
    Starbase,
    Sector,
    LoadingBayToIdleInput,
    StartMiningAsteroidInput,
    StopMiningAsteroidInput,
} from '@staratlas/sage';

import { SageGameHandler } from './sageGameHandler';

export class SageFleetHandler {
    constructor(
        private _gameHandler: SageGameHandler,
    ) {}

    async getFleetAccount(fleetPubkey: PublicKey): Promise<Fleet> {
        const fleet = readFromRPCOrError(
            this._gameHandler.provider.connection,
            this._gameHandler.program,
            fleetPubkey,
            Fleet,
            'confirmed',
        );

        return fleet;
    }

    async getMineItemAccount(mineItemPubkey: PublicKey): Promise<MineItem> {
        const mineItem = readFromRPCOrError(
            this._gameHandler.provider.connection,
            this._gameHandler.program,
            mineItemPubkey,
            MineItem,
            'confirmed',
        );

        return mineItem;
    }

    async getPlanetAccount(planetPubkey: PublicKey): Promise<Planet>  {
        const planet = readFromRPCOrError(
            this._gameHandler.provider.connection,
            this._gameHandler.program,
            planetPubkey,
            Planet,
            'confirmed',
        );

        return planet;
    }

    async getResourceAccount(resourcePubkey: PublicKey): Promise<Resource> {
        const resource = readFromRPCOrError(
            this._gameHandler.provider.connection,
            this._gameHandler.program,
            resourcePubkey,
            Resource,
            'confirmed',
        );

        return resource;
    }

    async getSectorAccount(sectorPubkey: PublicKey): Promise<Sector> {
        const sector = readFromRPCOrError(
            this._gameHandler.provider.connection,
            this._gameHandler.program,
            sectorPubkey,
            Sector,
            'confirmed',
        );

        return sector;
    }

    async getStarbaseAccount(starbasePubkey: PublicKey): Promise<Starbase> {
        const starbase = readFromRPCOrError(
            this._gameHandler.provider.connection,
            this._gameHandler.program,
            starbasePubkey,
            Starbase,
            'confirmed',
        );

        return starbase;
    }

    async ixDockToStarbase(fleetPubkey: PublicKey): Promise<InstructionReturn[]> {
        const fleetAccount = await this.getFleetAccount(fleetPubkey);

        // TODO: ensure fleet state is "Idle" - is there a better way to do this?
        if (!fleetAccount.state.Idle && !this._gameHandler.game) {
            throw 'fleet is not idle (or game is not loaded)';
        }

        const ixs: InstructionReturn[] = [];

        const coordinates = fleetAccount.state.Idle?.sector as [BN, BN];

        const starbaseKey = await this._gameHandler.getStarbaseAddress(coordinates);
        const starbaseAccount = await this.getStarbaseAccount(starbaseKey);

        const playerProfile = fleetAccount.data.ownerProfile;
        const sagePlayerProfile = await this._gameHandler.getSagePlayerProfileAddress(playerProfile);
        const starbasePlayerKey = await this._gameHandler.getStarbasePlayerAddress(starbaseKey, sagePlayerProfile, starbaseAccount.data.seqId);

        const program = this._gameHandler.program;
        const key = this._gameHandler.funder;
        const profileFaction = this._gameHandler.getProfileFactionAddress(playerProfile);
        const fleetKey = fleetAccount.key;
        const gameId = this._gameHandler.gameId as PublicKey;
        const gameState = this._gameHandler.gameState as PublicKey;
        const input = 0 as LoadingBayToIdleInput; // TODO: when would this change?

        const ix_1 = Fleet.idleToLoadingBay(
            program,
            key,
            playerProfile,
            profileFaction,
            fleetKey,
            starbaseKey,
            starbasePlayerKey,
            gameId,
            gameState,
            input,
        );

        ixs.push(ix_1);

        return ixs;
    }

    async ixUndockFromStarbase(fleetPubkey: PublicKey): Promise<InstructionReturn[]> {
        const fleetAccount = await this.getFleetAccount(fleetPubkey);

        // TODO: ensure fleet state is "StarbaseLoadingBay" - is there a better way to do this?
        if (!fleetAccount.state.StarbaseLoadingBay && !this._gameHandler.game) {
            throw 'fleet is not at starbase loading bay (or game is not loaded)';
        }

        const ixs: InstructionReturn[] = [];

        const starbaseKey = fleetAccount.state.StarbaseLoadingBay?.starbase as PublicKey;
        const starbaseAccount = await this.getStarbaseAccount(starbaseKey);

        const playerProfile = fleetAccount.data.ownerProfile;
        const sagePlayerProfile = await this._gameHandler.getSagePlayerProfileAddress(playerProfile);
        const starbasePlayerKey = await this._gameHandler.getStarbasePlayerAddress(starbaseKey, sagePlayerProfile, starbaseAccount.data.seqId);

        const program = this._gameHandler.program;
        const key = this._gameHandler.funder;
        const profileFaction = this._gameHandler.getProfileFactionAddress(playerProfile);
        const fleetKey = fleetAccount.key;
        const gameId = this._gameHandler.gameId as PublicKey;
        const gameState = this._gameHandler.gameState as PublicKey;
        const input = 0 as LoadingBayToIdleInput; // TODO: when would this change?

        const ix_1 = Fleet.loadingBayToIdle(
            program,
            key,
            playerProfile,
            profileFaction,
            fleetKey,
            starbaseKey,
            starbasePlayerKey,
            gameId,
            gameState,
            input,
        );

        ixs.push(ix_1);

        return ixs;
    }

    async ixStartMining(fleetPubkey: PublicKey, resource: string): Promise<InstructionReturn[]> {
        const fleetAccount = await this.getFleetAccount(fleetPubkey);

        // TODO: ensure fleet state is "Idle" - is there a better way to do this?
        if (!fleetAccount.state.Idle && !this._gameHandler.game) {
            throw 'fleet is not idle (or game is not loaded)';
        }

        const ixs: InstructionReturn[] = [];

        // TODO: is there a better way determine if anything is mineable (mint) at this 'location'?
        // see `getPlanetAddress` in sageGameHandler.ts (cache of planet addresses on load)
        const coordinates = fleetAccount.state.Idle?.sector as [BN, BN];

        const starbaseKey = await this._gameHandler.getStarbaseAddress(coordinates);
        const starbaseAccount = await this.getStarbaseAccount(starbaseKey);

        const playerProfile = fleetAccount.data.ownerProfile;
        const sagePlayerProfile = await this._gameHandler.getSagePlayerProfileAddress(playerProfile);
        const starbasePlayerKey = await this._gameHandler.getStarbasePlayerAddress(starbaseKey, sagePlayerProfile, starbaseAccount.data.seqId);
        const planetKey = await this._gameHandler.getPlanetAddress(starbaseAccount.data.sector as [BN, BN]);

        const mint = this._gameHandler.getResourceMintAddress(resource);

        if (!mint) {
            throw `resource mint not found for ${resource}`;
        }

        const mineItemKey = await this._gameHandler.getMineItemAddress(mint);
        const resourceKey = this._gameHandler.getResrouceAddress(mineItemKey, planetKey);

        const profileFaction = this._gameHandler.getProfileFactionAddress(playerProfile);
        const fleetKey = fleetAccount.key;

        const program = this._gameHandler.program;
        const key = this._gameHandler.funder;
        const gameState = this._gameHandler.gameState as PublicKey;
        const gameId = this._gameHandler.gameId as PublicKey;
        const input = { keyIndex: 0 } as StartMiningAsteroidInput;

        const ix_1 = Fleet.startMiningAsteroid(
            program,
            key,
            playerProfile,
            profileFaction,
            fleetKey,
            starbaseKey,
            starbasePlayerKey,
            mineItemKey,
            resourceKey,
            planetKey,
            gameState,
            gameId,
            input,
        );

        ixs.push(ix_1);

        return ixs;
    }

    async ixStopMining(fleetPubkey: PublicKey): Promise<InstructionReturn[]> {
        const fleetAccount = await this.getFleetAccount(fleetPubkey);

        // TODO: ensure fleet state is "MineAsteroid" - is there a better way to do this?
        if (!fleetAccount.state.MineAsteroid && !this._gameHandler.game) {
            throw 'fleet is not mining an asteroid (or game is not loaded)';
        }

        const ixs: InstructionReturn[] = [];

        const gameFoodMint = this._gameHandler.game?.data.mints.food as PublicKey;
        const gameAmmoMint = this._gameHandler.game?.data.mints.ammo as PublicKey;
        const gameFuelMint = this._gameHandler.game?.data.mints.fuel as PublicKey;

        const resourceKey = fleetAccount.state.MineAsteroid?.resource as PublicKey;
        const resourceAccount = await this.getResourceAccount(resourceKey);

        const mineItemKey = resourceAccount.data.mineItem; // TODO: check if this is the only way to get the 'mineItemKey'
        const mineItemAccount = await this.getMineItemAccount(mineItemKey);
        const mint = mineItemAccount.data.mint; // TODO: check if this is the only way get the 'mint'

        const planetKey = fleetAccount.state.MineAsteroid?.asteroid as PublicKey;
        const planetAccount = await this.getPlanetAccount(planetKey);

        const coordinates = planetAccount.data.sector as [BN, BN]; // TODO: check if this is the only way get the 'coordinates'
        const starbaseKey = await this._gameHandler.getStarbaseAddress(coordinates);

        const cargoHold = fleetAccount.data.cargoHold;
        const fleetAmmoBank = fleetAccount.data.ammoBank;
        const fleetFuelTank = fleetAccount.data.fuelTank;

        const resourceTokenFrom = await getAssociatedTokenAddress(mint, mineItemKey, true);
        const ataResourceTokenTo = await createAssociatedTokenAccountIdempotent(mint, cargoHold, true);
        const resourceTokenTo = ataResourceTokenTo.address;
        const ix_0 = ataResourceTokenTo.instructions;

        ixs.push(ix_0);

        const fleetFoodToken = await getAssociatedTokenAddress(gameFoodMint, cargoHold, true);
        const fleetAmmoToken = await getAssociatedTokenAddress(gameAmmoMint, fleetAmmoBank, true);
        const fleetFuelToken = await getAssociatedTokenAddress(gameFuelMint, fleetFuelTank, true);

        const program = this._gameHandler.program;
        const cargoProgram = this._gameHandler.cargoProgram;
        const playerProfile = fleetAccount.data.ownerProfile;
        const profileFaction = this._gameHandler.getProfileFactionAddress(playerProfile);
        const fleetKey = fleetAccount.key;
        const ammoBank = fleetAccount.data.ammoBank;
        const foodCargoType = this._gameHandler.getCargoTypeAddress(gameFoodMint);
        const ammoCargoType = this._gameHandler.getCargoTypeAddress(gameAmmoMint);
        const resourceCargoType = this._gameHandler.getCargoTypeAddress(mint);
        const cargoStatsDefinition = this._gameHandler.cargoStatsDefinition as PublicKey;
        const gameState = this._gameHandler.gameState as PublicKey;
        const gameId = this._gameHandler.gameId as PublicKey;
        const foodTokenFrom = fleetFoodToken;
        const ammoTokenFrom = fleetAmmoToken;
        const foodMint = gameFoodMint;
        const ammoMint = gameAmmoMint;

        const ix_1 = Fleet.asteroidMiningHandler(
            program,
            cargoProgram,
            profileFaction,
            fleetKey,
            starbaseKey,
            mineItemKey,
            resourceKey,
            planetKey,
            cargoHold,
            ammoBank,
            foodCargoType,
            ammoCargoType,
            resourceCargoType,
            cargoStatsDefinition,
            gameState,
            gameId,
            foodTokenFrom,
            ammoTokenFrom,
            resourceTokenFrom,
            resourceTokenTo,
            foodMint,
            ammoMint,
        );

        // // Debugging - tx instruction simulation errors
        // console.log("profileFaction", profileFaction.toBase58()); // Owner: pFACSRuobDmvfMKq1bAzwj27t6d2GJhSCHb1VcfnRmq
        // console.log("fleetKey", fleetKey.toBase58()); // Owner: SAGEqqFewepDHH6hMDcmWy7yjHPpyKLDnRXKb3Ki8e6
        // console.log("starbaseKey", starbaseKey.toBase58()); // Owner: SAGEqqFewepDHH6hMDcmWy7yjHPpyKLDnRXKb3Ki8e6
        // console.log("mineItemKey", mineItemKey.toBase58()); // Owner: SAGEqqFewepDHH6hMDcmWy7yjHPpyKLDnRXKb3Ki8e6
        // console.log("resourceKey", resourceKey.toBase58()); // Owner: SAGEqqFewepDHH6hMDcmWy7yjHPpyKLDnRXKb3Ki8e6
        // console.log("planetKey", planetKey.toBase58()); // Owner: SAGEqqFewepDHH6hMDcmWy7yjHPpyKLDnRXKb3Ki8e6
        // console.log("cargoHold", cargoHold.toBase58()); // Owner: Cargo8a1e6NkGyrjy4BQEW4ASGKs9KSyDyUrXMfpJoiH
        // console.log("ammoBank", ammoBank.toBase58()); // Owner: Cargo8a1e6NkGyrjy4BQEW4ASGKs9KSyDyUrXMfpJoiH
        // console.log("foodCargoType", foodCargoType.toBase58()); // Owner: Cargo8a1e6NkGyrjy4BQEW4ASGKs9KSyDyUrXMfpJoiH
        // console.log("ammoCargoType", ammoCargoType.toBase58()); // Owner: Cargo8a1e6NkGyrjy4BQEW4ASGKs9KSyDyUrXMfpJoiH
        // console.log("resourceCargoType", resourceCargoType.toBase58()); // Owner: Cargo8a1e6NkGyrjy4BQEW4ASGKs9KSyDyUrXMfpJoiH
        // console.log("cargoStatsDefinition", cargoStatsDefinition.toBase58()); // Owner: Cargo8a1e6NkGyrjy4BQEW4ASGKs9KSyDyUrXMfpJoiH
        // console.log("gameState", gameState.toBase58()); // Owner: SAGEqqFewepDHH6hMDcmWy7yjHPpyKLDnRXKb3Ki8e6
        // console.log("gameId", gameId.toBase58()); // Owner: SAGEqqFewepDHH6hMDcmWy7yjHPpyKLDnRXKb3Ki8e6
        // console.log("foodTokenFrom", foodTokenFrom.toBase58()); // Owner: cargoHold
        // console.log("ammoTokenFrom", ammoTokenFrom.toBase58()); // Owner: Cargo8a1e6NkGyrjy4BQEW4ASGKs9KSyDyUrXMfpJoiH
        // console.log("resourceTokenFrom", resourceTokenFrom.toBase58()); // Owner: mineItemKey
        // console.log("resourceTokenTo", resourceTokenTo.toBase58()); // Owner: ATA (mint, cargoHold)
        // console.log("foodMint", foodMint.toBase58()); // foodQJAztMzX1DKpLaiounNe2BDMds5RNuPC6jsNrDG
        // console.log("ammoMint", ammoMint.toBase58()); // ammoK8AkX2wnebQb35cDAZtTkvsXQbi82cGeTnUvvfK

        ixs.push(ix_1);

        const key = this._gameHandler.funder;
        const fuelTank = fleetFuelTank;
        const fuelCargoType = this._gameHandler.getCargoTypeAddress(gameFuelMint);
        const fuelTokenFrom = fleetFuelToken;
        const fuelMint = gameFuelMint;
        const input = { keyIndex: 0 } as StopMiningAsteroidInput;

        const ix_2 = Fleet.stopMiningAsteroid(
            program,
            cargoProgram,
            key,
            playerProfile,
            profileFaction,
            fleetKey,
            resourceKey,
            planetKey,
            fuelTank,
            fuelCargoType,
            cargoStatsDefinition,
            gameState,
            gameId,
            fuelTokenFrom,
            fuelMint,
            input,
        );

        ixs.push(ix_2);

        return ixs;
    }
}