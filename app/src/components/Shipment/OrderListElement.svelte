<script lang="ts">
	import type { ApiShipmentAccount } from '$src/utils/account/shipment';
	import clsx from 'clsx';
	import { createEventDispatcher } from 'svelte';

	export let shipmentAccount: ApiShipmentAccount;
	export let selectedAccount: string | undefined = undefined;

	const dispatch = createEventDispatcher();
	const handleShowClick = (e: MouseEvent) => {
		dispatch('buttonClicked');
	};

	$: shipmentData = shipmentAccount.account;
	$: locations = shipmentData.shipment.geography;
	$: priority = getPriorityName(shipmentData.shipment.details.priority);
	$: priorityColor = getPriorityColor(priority);
	$: statusNumber = shipmentData.status;
	$: status = getStatusString(statusNumber);

	function getStatusString(status: number) {
		switch (status) {
			case 5:
				return 'Delivered';
			case 4:
				return 'Accepted by carrier';
			case 3:
				return 'Offered to carrier';
			case 2:
				return 'Bought by forwarder';
			case 1:
				return 'Not yet bought';
			default:
				return 'Unknown';
		}
	}

	function getPriorityName(priority: number) {
		switch (priority) {
			case 4:
				return 'High';
			case 3:
				return 'Medium';
			case 2:
				return 'Low';
			default:
				return 'Default';
		}
	}

	function getPriorityColor(priority: string) {
		switch (priority) {
			case 'High':
				return 'text-red-600';
			case 'Medium':
				return 'text-yellow-600';
			case 'Low':
				return 'text-green-600';
			default:
				return 'text-gray-600';
		}
	}
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<li
	on:click
	class={clsx(
		'rounded-lg shadow cursor-pointer w-full',
		selectedAccount === shipmentAccount.publicKey ? 'bg-secondary-100' : 'bg-white'
	)}
>
	<div class="px-4 py-5 sm:px-6">
		<div class="flex items-center justify-between">
			<h3 class="sm:text-md xl:text-lg leading-6 font-medium text-gray-900">{shipmentData.name}</h3>
			<p
				class="mt-1 text-md bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent font-bold"
			>
				{shipmentData.price / 10 ** 9} SOL
			</p>
		</div>
		<div class="mt-3 xl:mt-5 flex items-center justify-between">
			<p class="text-xs xl:sm font-medium text-gray-500 mr-6 xl:mr-12">
				{locations.fromName + ' → ' + locations.toName}
				<br />
				&#x2022; Priority:
				<span class={clsx('font-semibold', priorityColor)}>{priority}</span>
				<br />
				&#x2022; Status:
				<span class={clsx('font-semibold')}>{status}</span>
			</p>

			<button class="text-sm xl:text-md text-accent font-medium" on:click={handleShowClick}
				>Show</button
			>
		</div>
	</div>
</li>
