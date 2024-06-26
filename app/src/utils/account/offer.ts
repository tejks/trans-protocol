import type BN from 'bn.js';
import type { ShipmentData } from './shipment';
import type { PublicKey } from '@solana/web3.js';
import type { ProgramAccount } from '@coral-xyz/anchor';
import type { ApiProgramAccount } from './common';

// u64 max cause someone set deadline to u64 max
export interface OfferDetails<BigNumber, U64MAX> {
	payment: BigNumber;
	deadline: U64MAX;
}

export interface ShipmentOfferStruct<Date, BigNumber, Key, U64MAX> {
	offeror: Key;
	shipment: Key;
	details: OfferDetails<BigNumber, U64MAX>;
	submitted: Date;
	timeout: Date;
	no: number;
	reserved: number[] //4
}

export type ShipmentOffer = ShipmentOfferStruct<BN, BN, PublicKey, BN>;
export type ApiShipmentOffer = ShipmentOfferStruct<string, number, string, string>;

export type ShipmentOfferAccount = ProgramAccount<ShipmentOffer>;

export type ApiShipmentOfferAccount = ApiProgramAccount<ApiShipmentOffer>;