import type { PublicKey } from '@solana/web3.js';
import type { Carrier } from '../account/carrier';
import type BN from 'bn.js';
import { decodeName } from '$sdk/sdk';
import type { AccountName } from '../account/common';

export function parseCarrierToApiCarrier(
	carrierAccount: Carrier<BN, PublicKey, AccountName>
): Carrier<string, string, string> {
	return {
		...carrierAccount,
		authority: carrierAccount.authority.toString(),
		creator: carrierAccount.creator.toString(),
		name: decodeName(carrierAccount.name),
		availability: {
			...carrierAccount.availability,
			locationName: decodeName(carrierAccount.availability.locationName),
			time: new Date(carrierAccount.availability.time.toNumber()).toISOString()
		}
	};
}