<script lang="ts">
	export let disabled: boolean = false;

	let className: string = '';
	export { className as class };
</script>

<button
	class="wallet-adapter-button {className}"
	{disabled}
	on:click
	on:focus
	on:mouseover
	on:mouseout
	on:blur
>
	{#if $$slots['start-icon']}
		<i class="wallet-adapter-button-start-icon">
			<slot name="start-icon" />
		</i>
	{/if}
	<slot />
	<span>
		<slot name="status" />
	</span>
</button>
