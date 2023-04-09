<script lang="ts">
	import type { HTMLInputAttributes, SVGAttributes } from 'svelte/elements';
	import MagnifyingGlass from './icons/MagnifyingGlass.svelte';
	import XMark from './icons/XMark.svelte';

	interface $$Props extends HTMLInputAttributes {
		value: string;
	}
	export let value: string;
</script>

<div
	class="bg-neutral-700/50 rounded grid grid-cols-[auto_1fr_auto] items-center text-neutral-300 focus-within:ring-2 ring-blue-500/75 border-b-2 border-neutral-700"
>
	<input
		type="text"
		bind:value
		{...$$restProps}
		class="bg-transparent flex-1 placeholder:text-neutral-600 focus:outline-none col-span-full row-start-1 px-6 text-sm cursor-default"
	/>
	<MagnifyingGlass class="h-4 w-4 mx-1 col-start-1 row-start-1 pointer-events-none" />
	<div class="bg-white rounded-full w-3 h-3 ml-1 mr-2 col-start-3 row-start-1">
		<XMark stroke-width="2" class="text-neutral-600" />
	</div>
</div>
