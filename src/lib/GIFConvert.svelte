<script>
	import { Command } from '@tauri-apps/api/shell';
	import { gifPath, outputPath } from '../stores';
	import { createDir, exists, writeTextFile } from '@tauri-apps/api/fs';
	import { animationName, fps, duration, cooldown } from '../stores';

	const convertGif = async () => {
		console.log($outputPath);
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
	};

	const createMeta = async (frameCount) => {
		console.log($outputPath);
		let frameOrder = '';
		for (let i = 0; i < frameCount; i++) {
			frameOrder += i + ' ';
		}
		frameOrder = frameOrder.trim();
		const metaText = `Filetype: FlipperAnimation
Version: 1

Width: 128
Height: 64
Passive frames ${frameCount}
Active frames: 0
Frames order: ${frameOrder}
Active cycles: 1
Frame rate: ${$fps}
Duration:${$duration}
Active cooldown: ${$cooldown}

Bubble slots: 0`;
		await writeTextFile($outputPath + '/' + $animationName + '/meta.txt', metaText);
	};
</script>

<button on:click={convertGif}>Convert GIF to Frames</button>
<p>${gifPath}</p>
