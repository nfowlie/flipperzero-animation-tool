<script>
	import { open } from '@tauri-apps/api/dialog';
	import { flipperzeroDir, outputPath } from '../stores.js';

	export let showModal;

	let dialog;

	$: if (dialog && showModal) dialog.showModal();
	$: if (dialog && !showModal) dialog.close();

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

<dialog
	bind:this={dialog}
	on:close={() => (showModal = false)}
	on:click|self={() => dialog.close()}
>
	<div on:click|stopPropagation>
		<h1>Select FlipperZero Firmware Directory</h1>
		<h2>Current FlipperZero Firmware Directory</h2>
		<button on:click={setFlipperZeroDir}>SELECT</button>
		<p>{$flipperzeroDir != null ? $flipperzeroDir : ``}</p>
	</div>
</dialog>
