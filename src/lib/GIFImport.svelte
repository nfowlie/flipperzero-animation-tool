<script>
	import { open } from '@tauri-apps/api/dialog';
	import { readDir, readBinaryFile, exists, createDir } from '@tauri-apps/api/fs';
	import { writable } from 'svelte/store';
	import { gifPath, outputPath, tempPath } from '../stores.js';
	import LoadingDialog from './LoadingDialog.svelte';
	import AnimationPreview from './AnimationPreview.svelte';
	import { parseGIF, decompressFrames } from 'gifuct-js';
	import { Command } from '@tauri-apps/api/shell';

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
			await exists($outputPath + '/temp').then((res) => {
				if (!res) createDir($outputPath + '/temp');
			});
			await new Command('graphics-magick', [
				'convert',
				$gifPath,
				'-resize',
				'128x64!',
				'-colorspace',
				'Gray',
				$outputPath + '/temp/resized.gif'
			]).execute();
			tempPath.set($outputPath + '/temp/resized.gif');
			const content = await readBinaryFile($tempPath);
			blob = new Blob([content], { type: 'image/gif' });
			promisedGif = await fetch(URL.createObjectURL(blob))
				.then((res) => res.arrayBuffer())
				.then((buff) => {
					const gif = parseGIF(buff);
					const frames = decompressFrames(gif, true);
					console.log(frames);
					return frames;
				});
			showLoading = false;
		} catch (err) {
			console.error(err);
		}
	};
</script>

<div class="gifImporter">
	<button on:click={getAnimationFile}>Select GIF</button>
	<AnimationPreview bind:gif={promisedGif} />
</div>

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
