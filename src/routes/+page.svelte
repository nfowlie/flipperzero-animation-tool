<script>
	import { run } from 'svelte/legacy';

	import GifInfo from '$lib/GIFInfo.svelte';
	import GIFImport from '$lib/GIFImport.svelte';
	import FlipperZero from '$lib/FlipperZero.svelte';
	import GifConvert from '$lib/GIFConvert.svelte';
	import { listen } from '@tauri-apps/api/event';
	import FirmwareModal from '$lib/FlipperZeroDialog.svelte';
	import { flipperzeroDir, outputPath } from '../stores';
	import '@picocss/pico';
	import UpdateDialog from '$lib/UpdateDialog.svelte';
	import AboutDialog from '$lib/AboutDialog.svelte';

	let showFirmwareModal = $state();
	let showAboutModal = $state();
	let showUpdateModal = $state();
	let appVersion = $state();

	listen('show_firmware_modal', (payload) => {
		showFirmwareModal = true;
	});

	listen('show_update_modal', (payload) => {
		console.log(payload);
		appVersion = payload.payload.message;
		console.log(appVersion);
		showUpdateModal = true;
	});

	listen('show_about_modal', (payload) => {
		console.log(payload);
		appVersion = payload.payload.message;
		console.log(appVersion);
		showAboutModal = true;
	});

	flipperzeroDir.set(localStorage.getItem('flipperzeroDir'));
	outputPath.set(localStorage.getItem('flipperzeroDir') + '/assets/dolphin/external');

	run(() => {
		($flipperzeroDir,
			(() => {
				showFirmwareModal = $flipperzeroDir == null ? true : false;
			})());
	});
</script>

<main>
	<h1>FlipperZero Animation Tool</h1>
	<GIFImport />
	<GifInfo />
</main>

<FirmwareModal bind:showFirmwareModal />
<UpdateDialog bind:showUpdateModal bind:appVersion />
<AboutDialog bind:showAboutModal bind:appVersion />

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
