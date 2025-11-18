<script>
	import { exists, readDir, remove } from '@tauri-apps/plugin-fs';
	import { open } from '@tauri-apps/plugin-dialog';
	import { Command } from '@tauri-apps/plugin-shell';
	import { flipperzeroDir } from '../stores';
	import { platform } from '@tauri-apps/plugin-os';

	export const convertFramesToFlipper = async () => {
		try {
			const os = await platform();
			const manifestExists = await exists(
				$flipperzeroDir + '/assets/resources/dolphin/manifest.txt'
			);
			if (manifestExists) await remove($flipperzeroDir + '/assets/resources/dolphin/manifest.txt');
			if (os == 'win32') {
				await new Command(
					'flipperzero-frames-win',
					['./fbt', 'icons', 'proto', 'dolphin_internal', 'dolphin_ext', 'resources'],
					{ cwd: $flipperzeroDir }
				).execute();
			} else {
				await new Command(
					'flipperzero-frames',
					['icons', 'proto', 'dolphin_internal', 'dolphin_ext', 'resources'],
					{ cwd: $flipperzeroDir }
				).execute();
			}
		} catch (err) {
			console.error(err);
		}
	};
</script>

<button form="gif-form" type="submit">Convert To Flipper Animation</button>
