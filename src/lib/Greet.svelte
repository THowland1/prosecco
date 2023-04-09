<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import type { UnicodeData } from './types';
	import Input from './Input.svelte';
	import { onMount } from 'svelte';

	let name = '';
	let greetMsg = '';
	let beep: UnicodeData[] = [];
	let selected: UnicodeData | null = {
		code_value: '1F6B1',
		character_name: 'NON-POTABLE WATER SYMBOL',
		general_category: 'So',
		canonical_combining_classes: '0',
		bidirectional_category: 'ON',
		character_decomposition_mapping: null,
		decimal_digit_value: null,
		digit_value: null,
		numeric_value: null,
		mirrored: 'N',
		unicode_1_0_name: null,
		iso_10646_comment_field: null,
		uppercase_mapping: null,
		lowercase_mapping: null,
		titlecase_mapping: null
	};
	$: {
		if (typeof name === 'string') {
			update();
		}
	}
	onMount(async () => {
		await update();
	});
	async function greet() {
		greetMsg = await invoke('greet', { name });
	}

	async function update() {
		try {
			const eee = await invoke<string>('unicode_data_list', {
				params: { top: 1000, query: name }
			});
			console.log(eee);
			beep = JSON.parse(eee) as UnicodeData[];
		} catch (e) {
			console.log(e);
		}
	}
</script>

<div class="select-none flex flex-col h-full">
	<div class="p-4">
		<Input
			id="greet-input"
			class="p-4"
			placeholder="Search"
			bind:value={name}
			on:change={() => update()}
		/>
	</div>
	<div class="flex-1 overflow-auto">
		<div class="flex flex-wrap p-4 pt-0">
			{#each beep as boop}
				<button
					class="w-10 h-10 rounded-lg flex flex-col justify-center items-center hover:bg-neutral-700 cursor-default"
					on:mouseenter={() => (selected = boop)}
				>
					<div class="w-8 h-8 flex flex-col justify-center items-center">
						{@html `&#x${boop.code_value}`}
					</div>
				</button>
			{/each}
		</div>
	</div>
	<div class="p-4 bg-neutral-800/10 border-t border-neutral-700">
		{#if selected}
			<div class="flex items-center">
				<div class="text-6xl mr-4 h-16 w-16 flex justify-center items-center">
					{@html `&#x${selected.code_value}`}
				</div>
				<div class="flex-1 text-neutral-300"><div>{selected.character_name}</div></div>
			</div>
		{/if}
	</div>
</div>
