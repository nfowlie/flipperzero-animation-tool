<script>
	import { Command } from '@tauri-apps/api/shell';
	import { gifPath, outputPath } from '../stores';

	const convertGif = async () => {
		console.log($outputPath);
		await new Command('graphics-magick', [
			'convert',
			$gifPath,
			'-resize',
			'128x64!',
			'-colorspace',
			'Gray',
			'-coalesce',
			'+adjoin',
			$outputPath + '/frame_%d.png'
		]).execute();
	};
</script>

<button on:click={convertGif}>Convert GIF to Frames</button>
<p>${gifPath}</p>
