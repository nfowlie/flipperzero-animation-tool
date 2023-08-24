<script>
	import { exists, readDir, removeFile } from '@tauri-apps/api/fs';
	import { open } from '@tauri-apps/api/dialog';
	import { Command } from '@tauri-apps/api/shell';
	import { flipperzeroDir } from '../stores';

	export const convertFramesToFlipper = async () => {
		try {
			const manifestExists = await exists(
				$flipperzeroDir + '/assets/resources/dolphin/manifest.txt'
			);
			if (manifestExists)
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

<button form="gif-form" type="submit">Convert To Flipper Animation</button>

<style>
	button {
		margin-inline: 1.2rem;
	}
</style>
