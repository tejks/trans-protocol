<script>
	import { page } from '$app/stores';
	import LayoutListWrapper from '$src/components/LayoutListWrapper.svelte';
	import { walletStore } from '$src/stores/wallet';
	import clsx from 'clsx';

	let isMobileOpen = false;
	$: isWalletConnected = $walletStore.publicKey != null;

	const routes = [
		{
			name: 'bought',
		},
		{
			name: 'offered',
		},
		{
			name: 'accepted',
		}
	];

	$: url = $page.url.pathname;
	$: forwarderPage = url.split('/').at(-1);
</script>

<LayoutListWrapper bind:isMobileOpen>
	{#if !isWalletConnected}
		<p
			class="mt-1 text-center text-xl bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent w-2/3"
		>
			Connect your wallet to view shipments
		</p>
	{:else}
		<div class="h-full flex w-full flex-col items-center">
			<div class="inline-flex shadow-sm bg-white rounded-lg m-4 flex-none">
				{#each routes as { name }, i}
					<a href={`/forwarder/${name}`}>
						<button
							aria-current="page"
							class={clsx(
								'px-4 py-2 text-md font-semibold',
								forwarderPage == name
									? 'bg-gradient-to-r from-primary to-secondary text-white'
									: 'bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent',
								i == 0 && 'rounded-l-lg',
								i == routes.length - 1 && 'rounded-r-lg'
							)}
						>
							{name}
						</button>
					</a>
				{/each}
			</div>
			<slot />
		</div>
	{/if}
</LayoutListWrapper>
