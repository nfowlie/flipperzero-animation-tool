<script>
	import { open } from '@tauri-apps/api/dialog';
	import { readDir, readBinaryFile } from '@tauri-apps/api/fs';
	import { writable } from 'svelte/store';
	import { gifPath } from '../stores.js';
	import LoadingDialog from './LoadingDialog.svelte';
	import AnimationPreview from './AnimationPreview.svelte';
	import { parseGIF, decompressFrames } from 'gifuct-js';

	let showLoading;

	let image;
	let blob;
	let promisedGif;
	const getAnimationFile = async () => {
		try {
			let selectedPath = await open({
				multiple: false,
				title: 'Open GIF File',
				filters: [{ name: 'Image', extensions: ['gif'] }]
			});
			if (!selectedPath) {
				showLoading = false;
				return;
			}
			showLoading = true;
			gifPath.set(selectedPath);
			const content = await readBinaryFile(selectedPath);
			blob = new Blob([content], { type: 'image/gif' });
			image.setAttribute('src', URL.createObjectURL(blob));
			promisedGif = await fetch(URL.createObjectURL(blob))
				.then((res) => res.arrayBuffer())
				.then((buff) => {
					const gif = parseGIF(buff);
					const frames = decompressFrames(gif, true);
					console.log(frames);
					return frames;
				});
			console.log(promisedGif);
			showLoading = false;
		} catch (err) {
			console.error(err);
		}
	};
</script>

<div class="gifImporter">
	<button on:click={getAnimationFile}>Select GIF</button>
	<img bind:this={image} alt="Selected GIF" height="64" width="128" />
</div>

<AnimationPreview bind:gif={promisedGif} />

<LoadingDialog bind:showLoading />

<style>
	button {
		margin-inline: 1.2rem;
	}
	img:not([src]) {
		display: none;
	}
	img {
		margin-bottom: 1.2rem;
	}
	.gifImporter {
		display: flex;
	}
</style>
