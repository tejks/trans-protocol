<script lang="ts">
	import { injectSpeedInsights } from '@vercel/speed-insights/sveltekit';

	injectSpeedInsights();
	// sass
	import '$src/sass/main.scss';

	//svelte
	import { onMount } from 'svelte';

	//wallets adapters
	import type { Adapter } from '@solana/wallet-adapter-base';
	import {
		Coin98WalletAdapter,
		LedgerWalletAdapter,
		PhantomWalletAdapter,
		SolflareWalletAdapter,
		TrezorWalletAdapter,
		TrustWalletAdapter
	} from '@solana/wallet-adapter-wallets';
	//wallet components
	import AnchorConnectionProvider from '$src/components/Wallet/AnchorConnectionProvider.svelte';
	import ConnectionProvider from '$src/components/Wallet/ConnectionProvider.svelte';
	import WalletProvider from '$src/components/Wallet/WalletProvider.svelte';
	// solana
	import { decodeName } from '$sdk/sdk';
	import LeftNavbar from '$src/components/Navigation/LeftNavbar.svelte';
	import Notifications from '$src/components/Notification/Notifications.svelte';
	import {
		createNotification,
		updateNotification
	} from '$src/components/Notification/notificationsStore';
	import MapWrapper from '$src/components/ShipmentMap/MapWrapper.svelte';
	import { fetchCarrierAccount } from '$src/lib/carrier';
	import { fetchForwarderAccount } from '$src/lib/forwarder';
	import { fetchShipperAccount } from '$src/lib/shipper';
	import { acceptedShipmentsOffersMeta } from '$src/stores/acceptedOffers';
	import { anchorStore } from '$src/stores/anchor';
	import { forwardedShipmentsMeta } from '$src/stores/forwarderShipments';
	import { shipmentsOffersMeta } from '$src/stores/offers';
	import { searchableShipments } from '$src/stores/searchableShipments';
	import { userStore } from '$src/stores/user';
	import { walletStore } from '$src/stores/wallet';
	import { web3Store } from '$src/stores/web3';
	import type {
		AcceptedShipmentOffer,
		ApiAcceptedShipmentOfferAccount
	} from '$src/utils/account/acceptedOffer';
	import type {
		ApiForwardedShipmentAccount,
		FetchedForwardedShipment
	} from '$src/utils/account/forwardedShipment';
	import type { ApiShipmentOfferAccount, ShipmentOffer } from '$src/utils/account/offer';
	import type { ApiShipmentAccount, FetchedShipment } from '$src/utils/account/shipment';
	import { parseAcceptedOfferToApiAcceptedOffer } from '$src/utils/parse/acceptedOffer';
	import { parseForwardedShipmentToApiForwardedShipment } from '$src/utils/parse/forwardedShipment';
	import { parseOfferToApiOffer } from '$src/utils/parse/offer';
	import { parseShipmentToApiShipment } from '$src/utils/parse/shipment';
	import { clusterApiUrl } from '@solana/web3.js';
	import { get } from 'svelte/store';

	let wallets: Adapter[];
	// it's a solana devnet cluster, but consider changing it to more performant provider
	const network = clusterApiUrl('devnet');
	const localStorageKey = 'walletAdapter';
	const { program } = get(anchorStore);
	const SOL_IN_LAMPORTS = 1000000000;
	const SHOULD_REQUEST_AIRDROP = false;
	$: isWalletConnected = $walletStore.publicKey !== null;

	const requiresAirdrop = async () => {
		const { connection } = get(web3Store);
		const { publicKey } = get(walletStore);

		const balance = await connection.getBalance(publicKey!);

		if (balance >= SOL_IN_LAMPORTS) {
			return false;
		}

		return true;
	};

	const airDropSol = async () => {
		const { connection } = get(web3Store);
		const { publicKey } = get(walletStore);

		const signature = await connection.requestAirdrop(publicKey!, SOL_IN_LAMPORTS);

		const latestBlockHash = await connection.getLatestBlockhash();

		await connection.confirmTransaction({
			blockhash: latestBlockHash.blockhash,
			lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
			signature
		});

		return signature;
	};

	$: if (isWalletConnected && SHOULD_REQUEST_AIRDROP) {
		requiresAirdrop().then((res) => {
			const id = createNotification({
				text: 'Airdrop',
				type: 'loading',
				removeAfter: 5000
			});
			if (res) {
				airDropSol()
					.then((signature) => {
						updateNotification(id, {
							text: 'Airdrop',
							type: 'success',
							removeAfter: 3000,
							signature
						});
					})
					.catch(() => {
						updateNotification(id, {
							text: 'Airdrop',
							type: 'failed',
							removeAfter: 3000
						});
					});
			}
		});
	}

	// check if user is registered as forwarder, shipper or carrier
	// this is done on wallet sign in/sign out, when user changes wallet
	$: if ($walletStore.publicKey) {
		fetchForwarderAccount(program, $walletStore.publicKey).then(({ account, accountKey }) => {
			if (account) {
				userStore.registerForwarder(decodeName(account.name), accountKey.toString());
			}
		});

		fetchShipperAccount(program, $walletStore.publicKey).then(({ account, accountKey }) => {
			if (account) {
				userStore.registerShipper(decodeName(account.name), accountKey.toString());
			}
		});

		fetchCarrierAccount(program, $walletStore.publicKey).then(({ account, accountKey }) => {
			if (account) {
				userStore.registerCarrier(decodeName(account.name), accountKey.toString());
			}
		});
	} else {
		userStore.unregisterForwarder();
		userStore.unregisterShipper();
		userStore.unregisterCarrier();
	}

	function subscribeToShipmentEvents(): number[] {
		const unsubscribeShipmentCreated = program.addEventListener(
			'ShipmentCreated',
			async (event) => {
				const shipmentPublicKey = event.shipment;

				const shipment: FetchedShipment = await program.account.shipment.fetch(shipmentPublicKey);

				const parsedShipment: ApiShipmentAccount = {
					publicKey: shipmentPublicKey.toString(),
					account: parseShipmentToApiShipment(shipment)
				};

				const shipper = event.shipper;

				searchableShipments.extend({
					...parsedShipment,
					searchParams: parsedShipment.account.shipment.details.priority.toString()
				});
			}
		);

		const unsubscribeForwardedShipment = program.addEventListener(
			'ShipmentTransferred',
			async (event) => {
				const forwardedShipmentPublicKey = event.forwarded;

				const forwardedShipment: FetchedForwardedShipment =
					await program.account.forwardedShipment.fetch(forwardedShipmentPublicKey);

				const parsedForwardedShipment: ApiForwardedShipmentAccount = {
					publicKey: forwardedShipmentPublicKey.toString(),
					account: parseForwardedShipmentToApiForwardedShipment(forwardedShipment)
				};

				const buyer = event.buyer;

				forwardedShipmentsMeta.update((meta) => {
					meta.push(parsedForwardedShipment);
					return meta;
				});

				searchableShipments.update((s) => {
					const shipmentIndex = s.data.findIndex((a) => a.publicKey === event.shipment.toString());

					const shipmentToChange = s.data[shipmentIndex];
					shipmentToChange.account.status = 2;
					shipmentToChange.account.forwarder = event.buyer.toString();

					return s;
				});
			}
		);

		const unsubscribeOfferedShipment = program.addEventListener('OfferMade', async (event) => {
			const offeredShipmentPublicKey = event.offer;

			const offeredShipment: ShipmentOffer =
				await program.account.shipmentOffer.fetch(offeredShipmentPublicKey);

			const parsedOfferedShipment: ApiShipmentOfferAccount = {
				publicKey: offeredShipmentPublicKey.toString(),
				account: parseOfferToApiOffer(offeredShipment)
			};

			const offerer = event.from;

			const shipment = event.shipment.toString();
			forwardedShipmentsMeta.update((meta) => {
				const forwardIndex = meta.findIndex((f) => f.account.shipment === shipment);

				if (forwardIndex != -1) {
					meta.splice(forwardIndex, 1);
				}

				return meta;
			});

			shipmentsOffersMeta.update((o) => {
				o.push(parsedOfferedShipment);

				return o;
			});

			searchableShipments.update((s) => {
				const shipmentIndex = s.data.findIndex((a) => a.publicKey === event.shipment.toString());

				if (shipmentIndex != -1) {
					s.data[shipmentIndex].account.status = 3;
				}

				return s;
			});
		});

		const unsubscribeAcceptedShipment = program.addEventListener('OfferAccepted', async (event) => {
			const offeredShipmentPublicKey = event.offer;

			const acceptedShipment: AcceptedShipmentOffer =
				await program.account.acceptedOffer.fetch(offeredShipmentPublicKey);

			const parsedAcceptedShipment: ApiAcceptedShipmentOfferAccount = {
				publicKey: offeredShipmentPublicKey.toString(),
				account: parseAcceptedOfferToApiAcceptedOffer(acceptedShipment)
			};

			const carrier = event.to;

			const shipment = event.shipment.toString();
			shipmentsOffersMeta.update((meta) => {
				const offerIndex = meta.findIndex((f) => f.account.shipment === shipment);

				if (offerIndex != -1) {
					meta.splice(offerIndex, 1);
				}

				return meta;
			});

			acceptedShipmentsOffersMeta.update((o) => {
				o.push(parsedAcceptedShipment);

				return o;
			});

			const shipmentPublicKey = event.shipment;

			const shipmentAccount: FetchedShipment =
				await program.account.shipment.fetch(shipmentPublicKey);

			const parsedShipment: ApiShipmentAccount = {
				publicKey: shipmentPublicKey.toString(),
				account: parseShipmentToApiShipment(shipmentAccount)
			};

			searchableShipments.update((s) => {
				const shipmentIndex = s.data.findIndex((a) => a.publicKey === event.shipment.toString());

				if (shipmentIndex != -1) {
					s.data[shipmentIndex].account = parsedShipment.account;
				}

				return s;
			});
		});

		return [
			unsubscribeForwardedShipment,
			unsubscribeShipmentCreated,
			unsubscribeOfferedShipment,
			unsubscribeAcceptedShipment
		];
	}

	onMount(() => {
		wallets = [
			new PhantomWalletAdapter(),
			new SolflareWalletAdapter(),
			new TrustWalletAdapter(),
			new Coin98WalletAdapter(),
			new LedgerWalletAdapter(),
			new TrezorWalletAdapter()
		];

		const unsubscribe = subscribeToShipmentEvents();
		return () => {
			for (const listener of unsubscribe) {
				program.removeEventListener(listener);
			}
		};
	});
</script>

<AnchorConnectionProvider {network} />
<ConnectionProvider {network} />
<WalletProvider {localStorageKey} {wallets} autoConnect />

<Notifications>
	<LeftNavbar />

	<MapWrapper>
		<slot />
	</MapWrapper>
</Notifications>
