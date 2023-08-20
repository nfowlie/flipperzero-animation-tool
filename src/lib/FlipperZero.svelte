<script>
	import { readDir } from '@tauri-apps/api/fs';
	import { open } from '@tauri-apps/api/dialog';
	import { Command } from '@tauri-apps/api/shell';
	import { writable } from 'svelte/store';

	export const flipperZeroDir = writable('');

	const setFlipperZeroDir = async () => {
		try {
			const selectedPath = await open({ directory: true, multiple: false });
			flipperZeroDir.update(() => selectedPath);
			if (!selectedPath) return;
		} catch (err) {
			console.error(err);
		}
	};

	const convertFramesToFlipper = async () => {
		try {
			await new Command(
				'flipperzero-frames',
				['icons', 'proto', 'dolphin_internal', 'dolphin_ext', 'resources'],
				{ cwd: $flipperZeroDir }
			).execute();
		} catch (err) {
			console.error(err);
		}
	};
</script>

<button on:click={setFlipperZeroDir}>Select FlipperZero Directory</button>
<p>{$flipperZeroDir}</p>
<button on:click={convertFramesToFlipper}>Convert To Flipper Animation</button>
