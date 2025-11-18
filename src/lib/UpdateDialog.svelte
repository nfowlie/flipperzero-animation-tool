<script lang="ts">
	import LoadingDialog from './LoadingDialog.svelte';
	import { run, createBubbler, stopPropagation } from 'svelte/legacy';
	import { openUrl } from '@tauri-apps/plugin-opener';

	let showLoading = $state();
	let upToDate = $state();
	let updateLink = $state();
	let updateVersion = $state();

	let { showUpdateModal = $bindable(), appVersion = $bindable() } = $props();

	const checkUpdated = async () => {
		showLoading = true;
		fetch('https://api.github.com/repos/nfowlie/flipperzero-animation-tool/releases')
			.then((res) => res.json())
			.then((releases) => {
				setTimeout(() => {
					showLoading = false;
				}, 400);
				updateLink = releases[0].html_url;
				updateVersion = releases[0].tag_name.substring(1);
				if (updateVersion === appVersion) upToDate = true;
				else upToDate = false;
			})
			.catch((err) => {
				console.error(err);
			});
	};

	$effect(() => {
		console.log(showUpdateModal);
		if (showUpdateModal === true) checkUpdated();
	});

	const bubble = createBubbler();
	import { open } from '@tauri-apps/plugin-dialog';

	let dialog = $state();

	run(() => {
		if (dialog && showUpdateModal) dialog.showModal();
	});
	run(() => {
		if (dialog && !showUpdateModal) dialog.close();
	});

	const handleUpdateLink = async () => {
		await openUrl(updateLink);
	};

	const handleDialogClose = () => {
		showUpdateModal = false;
		dialog.close();
	};
</script>

<dialog bind:this={dialog} onclose={() => (showUpdateModal = false)} onclick={handleDialogClose}>
	<div onclick={stopPropagation(bubble('click'))}>
		<h1>FlipperZero Animation Tool</h1>
		{#if upToDate}
			<h2>Up To Date - Version: {appVersion}</h2>
		{:else}
			<h2>Update Available - <a href="#" onclick={handleUpdateLink}>{updateVersion}</a></h2>
		{/if}
		<button onclick={handleDialogClose}>CLOSE</button>
	</div>
</dialog>

<LoadingDialog bind:showLoading />

<style>
	dialog {
		text-align: center;
	}
</style>
