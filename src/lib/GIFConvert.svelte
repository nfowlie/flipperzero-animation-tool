<script>
	import { Command } from '@tauri-apps/api/shell';
	import { gifPath, outputPath } from '../stores';
	import { createDir, exists, readTextFile, writeTextFile } from '@tauri-apps/api/fs';
	import {
		flipperzeroDir,
		animationName,
		fps,
		duration,
		cooldown,
		minButthurt,
		maxButthurt,
		minLevel,
		maxLevel,
		weight
	} from '../stores';
	import FlipperZero from './FlipperZero.svelte';
	let flipperzero;

	export const convertGif = async () => {
		try {
			if (!$gifPath) throw new Error('No Gif Selected');

			const frameCount = await new Command('graphics-magick', ['identify', $gifPath])
				.execute()
				.then((res) => {
					return res.stdout.split(`\n`).length;
				});

			await exists($outputPath + '/' + $animationName).then((res) => {
				if (!res) createDir($outputPath + '/' + $animationName);
			});

			await new Command('graphics-magick', [
				'convert',
				$gifPath,
				'-resize',
				'128x64!',
				'-colorspace',
				'Gray',
				'-coalesce',
				'+adjoin',
				$outputPath + '/' + $animationName + '/frame_%d.png'
			]).execute();
			createMeta(frameCount);
		} catch (err) {
			console.error(err);
			alert(err);
		}
	};

	const createMeta = async (frameCount) => {
		try {
			let frameOrder = '';
			for (let i = 0; i < frameCount; i++) {
				frameOrder += i + ' ';
			}
			frameOrder = frameOrder.trim();
			const metaText =
				'Filetype: Flipper Animation\nVersion: 1\n\nWidth: 128\nHeight: 64\nPassive frames: ' +
				(frameCount - 1) +
				'\nActive frames: 1\nFrames order: ' +
				frameOrder +
				'\nActive cycles: 1\nFrame rate: ' +
				$fps +
				'\nDuration: ' +
				$duration +
				'\nActive cooldown: ' +
				$cooldown +
				'\n\nBubble slots: 0\n';
			await writeTextFile($outputPath + '/' + $animationName + '/meta.txt', metaText);
			editManifest();
		} catch (err) {
			console.error('Error creating Meta.txt for animation');
			alert('Error creating Meta.txt for animation');
		}
	};

	const editManifest = async () => {
		try {
			const manifest = await readTextFile(
				$flipperzeroDir + '/assets/dolphin/external/manifest.txt'
			);
			const matchAnimationName = 'Name: ' + $animationName + '\n';
			const regex = new RegExp(matchAnimationName);
			const existsInManifest = manifest.match(regex);
			if (existsInManifest) {
				throw new Error('Animation with that name already exists');
			} else {
				const newAnimationManifest =
					'\nName: ' +
					$animationName +
					'\nMin butthurt: ' +
					$minButthurt +
					'\nMax butthurt: ' +
					$maxButthurt +
					'\nMin level: ' +
					$minLevel +
					'\nMax level: ' +
					$maxLevel +
					'\nWeight: ' +
					$weight +
					'\n';

				await writeTextFile(
					$flipperzeroDir + '/assets/dolphin/external/manifest.txt',
					manifest + newAnimationManifest
				);
				flipperzero.convertFramesToFlipper();
			}
		} catch (err) {
			if (err) {
				console.error(err);
				alert(err);
			} else {
				console.error('Error Updating Manifest.txt');
				alert('Error Updating Manifest.txt');
			}
		}
	};
</script>

<FlipperZero bind:this={flipperzero} />
