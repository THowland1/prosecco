<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	let name = '';
	let greetMsg = '';
	let beep: string[] = ['fds'];

	async function greet() {
		greetMsg = await invoke('greet', { name });

		try {
			const eee = await invoke('unicode_data_list');
			beep = [...beep, JSON.stringify(eee)];
		} catch (e) {
			beep = [...beep, JSON.stringify(e)];
		}
	}
</script>

<div>
	<input id="greet-input" placeholder="Enter a name..." bind:value={name} />
	<button on:click={greet}>Greet</button>
	<p>{greetMsg}</p>
	<ul>
		{#each beep as boop}
			<li>{boop}</li>
		{/each}
	</ul>
	<p />
</div>
