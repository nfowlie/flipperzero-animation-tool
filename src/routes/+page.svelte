<script>
	import GifInfo from '$lib/GIFInfo.svelte';
	import GIFImport from '$lib/GIFImport.svelte';
	import FlipperZero from '$lib/FlipperZero.svelte';
	import GifConvert from '$lib/GIFConvert.svelte';
	import { listen } from '@tauri-apps/api/event';
	import Modal from '$lib/FlipperZeroDialog.svelte';
	import { flipperzeroDir, outputPath } from '../stores';
	import '@picocss/pico';
	// import Dys from '../../static/fonts/LigaOpenDyslexic3-Regular.ttf';

	const unlisten = listen('my_event', (payload) => {
		showModal = true;
	});

	let showModal = false;
	flipperzeroDir.set(localStorage.getItem('flipperzeroDir'));
	outputPath.set(localStorage.getItem('flipperzeroDir') + '/assets/dolphin/external');

	$: $flipperzeroDir,
		(() => {
			showModal = $flipperzeroDir === null ? true : false;
		})();
</script>

<h1>FlipperZero Animation Tool</h1>
<GIFImport />
<GifInfo />

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

	h1 {
		margin-inline: 1.2rem;
	}
</style>
