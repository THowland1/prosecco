<script>
	import { notifications } from '$lib/stores/toast';
	import { slide } from 'svelte/transition';
	import '../app.css';
	import Check from '$lib/icons/Check.svelte';
</script>

<div class="absolute inset-0 bg-neutral-800 text-neutral-200 grid">
	<div class="col-start-1 row-start-1 overflow-auto">
		<slot />
	</div>
	<div class="absolute top-4 left-0 right-0 mx-auto pointer-events-none max-h-full overflow-hidden">
		<div class="w-fit mx-auto flex flex-col">
			{#each $notifications as notification (notification.id)}
				<div
					transition:slide
					class="bg-neutral-800 shadow-lg border border-neutral-600 mx-auto py-1 px-2 text-sm text-neutral-300 rounded mb-2 inline-flex items-center"
				>
					<div
						class="w-4 h-4 bg-green-600 text-white rounded-full mr-2 inline-flex items-center justify-center"
					>
						<Check class="w-3 h-3" />
					</div>
					<div>
						{notification.message}
					</div>
				</div>
			{/each}
		</div>
	</div>
</div>
