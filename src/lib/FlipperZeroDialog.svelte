<script lang="ts">
	import { run, self, createBubbler, stopPropagation } from 'svelte/legacy';

	const bubble = createBubbler();
	import { open } from '@tauri-apps/plugin-dialog';
	import { flipperzeroDir, outputPath } from '../stores.js';

	let { showModal = $bindable() } = $props();

	let dialog = $state();

	run(() => {
		if (dialog && showModal) dialog.showModal();
	});
	run(() => {
		if (dialog && !showModal) dialog.close();
	});

	const setFlipperZeroDir = async () => {
		try {
			const selectedPath = await open({ directory: true, multiple: false });
			flipperzeroDir.set(selectedPath);
			outputPath.set(selectedPath + '/assets/dolphin/external');
			localStorage.setItem('flipperzeroDir', $flipperzeroDir);
			if (!selectedPath) return;
		} catch (err) {
			console.error(err);
		}
	};
</script>

<dialog bind:this={dialog} onclose={() => (showModal = false)} onclick={self(() => dialog.close())}>
	<div onclick={stopPropagation(bubble('click'))}>
		<h1>Select FlipperZero Firmware Directory</h1>
		<h2>Current FlipperZero Firmware Directory</h2>
		<button onclick={setFlipperZeroDir}>SELECT</button>
		<p>{$flipperzeroDir != null ? $flipperzeroDir : ``}</p>
	</div>
</dialog>
