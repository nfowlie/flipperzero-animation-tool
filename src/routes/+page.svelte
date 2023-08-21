<script>
	import GIFImport from '$lib/GIFImport.svelte';
	import FlipperZero from '$lib/FlipperZero.svelte';
	import GifConvert from '$lib/GIFConvert.svelte';
	import { listen } from '@tauri-apps/api/event';
	import Modal from '$lib/FlipperZeroDialog.svelte';
	import { flipperzeroDir, outputPath } from '../stores';
	import '@picocss/pico';

	const unlisten = listen('my_event', (payload) => {
		showModal = true;
	});

	let showModal = false;
	flipperzeroDir.set(localStorage.getItem('flipperzeroDir'));
	outputPath.set(localStorage.getItem('flipperzeroDir') + '/assets/dolphin/external/test');

	$: $flipperzeroDir,
		(() => {
			showModal = $flipperzeroDir === null ? true : false;
		})();
</script>

<h1>FlipperZero Animation Tool</h1>

<GIFImport />
<GifConvert />
<FlipperZero />

<Modal bind:showModal />
