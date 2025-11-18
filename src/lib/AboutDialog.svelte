<script lang="ts">
	import { run, self, createBubbler, stopPropagation } from 'svelte/legacy';

	const bubble = createBubbler();
	import { open } from '@tauri-apps/plugin-dialog';
	import { flipperzeroDir, outputPath } from '../stores.js';

	let { showAboutModal = $bindable(), appVersion = $bindable() } = $props();

	let dialog = $state();

	run(() => {
		if (dialog && showAboutModal) dialog.showModal();
	});
	run(() => {
		if (dialog && !showAboutModal) dialog.close();
	});
</script>

<dialog
	bind:this={dialog}
	onclose={() => (showAboutModal = false)}
	onclick={self(() => dialog.close())}
>
	<div onclick={stopPropagation(bubble('click'))}>
		<h1>FlipperZero Animation Tool</h1>
		<h2>version: {appVersion}</h2>
		<button onclick={self(() => dialog.close())}>CLOSE</button>
	</div>
</dialog>

<style>
	dialog {
		text-align: center;
	}
</style>
