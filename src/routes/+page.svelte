<script>
	import { run } from 'svelte/legacy';

	import GifInfo from '$lib/GIFInfo.svelte';
	import GIFImport from '$lib/GIFImport.svelte';
	import FlipperZero from '$lib/FlipperZero.svelte';
	import GifConvert from '$lib/GIFConvert.svelte';
	import { listen } from '@tauri-apps/api/event';
	import Modal from '$lib/FlipperZeroDialog.svelte';
	import { flipperzeroDir, outputPath } from '../stores';
	import '@picocss/pico';
	// import Dys from '../../static/fonts/LigaOpenDyslexic3-Regular.ttf';

	let showModal = $state();
	const unlisten = listen('show_modal', (payload) => {
		showModal = true;
	});

	flipperzeroDir.set(localStorage.getItem('flipperzeroDir'));
	outputPath.set(localStorage.getItem('flipperzeroDir') + '/assets/dolphin/external');

	run(() => {
		($flipperzeroDir,
			(() => {
				showModal = $flipperzeroDir == null ? true : false;
			})());
	});
</script>

<main>
	<h1>FlipperZero Animation Tool</h1>
	<GIFImport />
	<GifInfo />
</main>

<Modal bind:showModal />

<style>
	:global(*),
	*,
	:global(h1, h2) {
		font-family: 'Born2bSportyV2';
		color: var(--main-color);
		background-color: var(--sesondary-color);
	}
	:global(button) {
		color: var(--main-color);
		background-color: var(--transparency-color);
		border-color: var(--main-color);
		width: fit-content;
		height: fit-content;
	}

	main {
		display: grid;
		margin-block: calc(var(--spacing-block) / 2);
		margin-inline: var(--spacing-inline);
		gap: var(--spacing-block);
	}
</style>
