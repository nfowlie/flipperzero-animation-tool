<script>
	import { open } from '@tauri-apps/api/dialog';
	import { flipperzeroDir } from '../stores.js';

	export let showModal;

	let dialog;

	$: if (dialog && showModal) dialog.showModal();
	$: if (dialog && !showModal) dialog.close();

	const setFlipperZeroDir = async () => {
		try {
			const selectedPath = await open({ directory: true, multiple: false });
			flipperzeroDir.set(selectedPath);
			localStorage.setItem('flipperzeroDir', $flipperzeroDir);
			if (!selectedPath) return;
		} catch (err) {
			console.error(err);
		}
	};
</script>

<dialog bind:this={dialog} on:close={() => (showModal = false)}>
	<h1>Select FlipperZero Firmware Directory</h1>
	<h2>Current FlipperZero Firmware Directory</h2>
	<p>{$flipperzeroDir}</p>
	<button on:click={setFlipperZeroDir}>SELECT</button>
</dialog>
