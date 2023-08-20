<script>
	import { open } from '@tauri-apps/api/dialog';
	import { readDir, readBinaryFile } from '@tauri-apps/api/fs';
	import { writable } from 'svelte/store';
	import { gifPath } from '../stores.js';

	let image;
	const getAnimationFile = async () => {
		try {
			let selectedPath = await open({
				multiple: false,
				title: 'Open GIF File',
				filters: [{ name: 'Image', extensions: ['gif'] }]
			});
			console.log(selectedPath);
			gifPath.set(selectedPath);
			const content = await readBinaryFile(selectedPath);
			const blob = new Blob([content], { type: 'image/gif' });
			image.setAttribute('src', URL.createObjectURL(blob));
			if (!selectedPath) return;
		} catch (err) {
			console.error(err);
		}
	};
</script>

<div class="gifImporter">
	<button on:click={getAnimationFile}>Select GIF</button>
	<img bind:this={image} alt="selected gif" height="64" width="128" />
</div>

<style>
	.gifImporter {
		display: flex;
		flex-direction: column;
	}
	.gifImporter > button {
		margin-block-end: 1.2rem;
	}
</style>
