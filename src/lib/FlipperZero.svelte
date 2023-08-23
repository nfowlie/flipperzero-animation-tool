<script>
	import { readDir, removeFile } from '@tauri-apps/api/fs';
	import { open } from '@tauri-apps/api/dialog';
	import { Command } from '@tauri-apps/api/shell';
	import { flipperzeroDir } from '../stores';

	const convertFramesToFlipper = async () => {
		try {
			await removeFile($flipperzeroDir + '/assets/resources/dolphin/manifest.txt');
			await new Command(
				'flipperzero-frames',
				['icons', 'proto', 'dolphin_internal', 'dolphin_ext', 'resources'],
				{ cwd: $flipperzeroDir }
			).execute();
		} catch (err) {
			console.error(err);
		}
	};
</script>

<button on:click={convertFramesToFlipper}>Convert To Flipper Animation</button>
