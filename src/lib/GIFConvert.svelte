<script>
	import { Command } from '@tauri-apps/plugin-shell';
	import { gifPath, outputPath, tempPath } from '../stores';
	import { create, mkdir, exists, readTextFile, writeTextFile } from '@tauri-apps/plugin-fs';
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
		weight,
		textBoxX,
		textBoxY,
		alignH,
		alignV,
		startFrame,
		endFrame,
		bubbleText,
		bubbleTextPresent
	} from '../stores';
	import FlipperZero from './FlipperZero.svelte';
	import LoadingDialog from './LoadingDialog.svelte';
	let flipperzero = $state();
	let showLoading = $state();

	export const convertGif = async () => {
		try {
			showLoading = true;
			if ($flipperzeroDir === null) throw new Error('FlipperZero Firmware Directory not selected');
			if ($flipperzeroDir !== null) {
				const fbtExists = await exists($flipperzeroDir + '/fbt');
				if (!fbtExists)
					throw new Error('FlipperZero Firmware Directory selected is not a Firmware Directory');
			}
			if (!$gifPath) throw new Error('No Gif Selected');
			const animationExists = await exists($outputPath + '/' + $animationName);
			if (animationExists) throw new Error('An animation with that name already exists');
			const frameCount = await new Command('graphics-magick', ['identify', $tempPath])
				.execute()
				.then((res) => {
					return res.stdout.split(`\n`).length;
				});

			await exists($outputPath + '/' + $animationName).then((res) => {
				if (!res) mkdir($outputPath + '/' + $animationName);
			});

			await new Command('graphics-magick', [
				'convert',
				$tempPath,
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
			showLoading = false;
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
			let bubbleCount = $bubbleTextPresent ? 1 : 0;
			let metaTextBubble = '';
			if ($bubbleTextPresent) {
				metaTextBubble =
					'\nSlot: 0\nX: ' +
					$textBoxX +
					'\nY: ' +
					$textBoxY +
					'\nText: ' +
					$bubbleText.replace('\n', '\\n') +
					'\nAlignH: ' +
					$alignH +
					'\nAlignV: ' +
					$alignV +
					'\nStartFrame: ' +
					$startFrame +
					'\nEndFrame: ' +
					$endFrame +
					'\n';
			}
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
				'\n\nBubble slots: ' +
				bubbleCount +
				'\n' +
				metaTextBubble;

			console.log($outputPath + '/' + $animationName + '/' + 'meta.txt');
			const file = await create(`${$outputPath}/${$animationName}/meta.txt`);
			await file.write(new TextEncoder().encode('metaText'));
			await file.close();

			// await createTextFile($outputPath + '/' + $animationName + '/meta.txt');
			// await writeTextFile($outputPath + '/' + $animationName + '/meta.txt', metaText);
			editManifest();
		} catch (err) {
			console.error('Error creating Meta.txt for animation');
			console.error(err);
			showLoading = false;
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
				flipperzero
					.convertFramesToFlipper()
					.then(() => {
						showLoading = false;
						alert('Animation Complete');
					})
					.catch((err) => {
						showLoading = false;
						console.error(err);
						alert('Error exporting frames');
					});
			}
		} catch (err) {
			showLoading = false;
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
<LoadingDialog bind:showLoading />
