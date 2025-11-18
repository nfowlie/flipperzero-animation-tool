<script>
	import { open } from '@tauri-apps/plugin-dialog';
	import { readDir, readFile, exists, mkdir } from '@tauri-apps/plugin-fs';
	import { writable } from 'svelte/store';
	import { gifPath, outputPath, tempPath, gifFrameLength } from '../stores.js';
	import LoadingDialog from './LoadingDialog.svelte';
	import AnimationPreview from './AnimationPreview.svelte';
	import { parseGIF, decompressFrames } from 'gifuct-js';
	import { Command } from '@tauri-apps/plugin-shell';

	let showLoading = $state();

	let image;
	let blob;
	let promisedGif = $state();
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
				if (!res) mkdir($outputPath + '/temp');
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
			console.log(tempPath);
			tempPath.set($outputPath + '/temp/resized.gif');
			const content = await readFile($tempPath);
			blob = new Blob([content], { type: 'image/gif' });
			promisedGif = await fetch(URL.createObjectURL(blob))
				.then((res) => res.arrayBuffer())
				.then((buff) => {
					const gif = parseGIF(buff);
					const frames = decompressFrames(gif, true);
					console.log(frames);
					return frames;
				});
			gifFrameLength.set(promisedGif.length);
			showLoading = false;
		} catch (err) {
			console.error(err);
		}
	};
</script>

<div class="gifImporter">
	<button onclick={getAnimationFile}>Select GIF</button>
	<AnimationPreview bind:gif={promisedGif} />
</div>

<LoadingDialog bind:showLoading />

<style>
	.gifImporter {
		display: flex;
		gap: var(--spacing-inline);
	}
	img:not([src]) {
		display: none;
	}
</style>
