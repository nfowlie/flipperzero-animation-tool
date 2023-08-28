<script>
	import { parseGIF, decompressFrames } from 'gifuct-js';
	import { onMount } from 'svelte';
	import { fps } from '../stores';

	export let gif;
	let loadedFrames,
		frameIndex = 0,
		needsDisposal = false,
		frameImageData,
		canvasHeight = 0,
		canvasWidth = 0,
		canvasPreview,
		canvasContext;
	$: if (gif) renderGif(gif);
	$: $fps,
		() => {
			if (gif) renderGif(gif);
		};

	onMount(() => {
		canvasPreview = document.querySelector('canvas');
		canvasContext = canvasPreview?.getContext('2d');
	});

	const tempCanvas = document.createElement('canvas');
	const tempContext = tempCanvas.getContext('2d');

	const gifCanvas = document.createElement('canvas');
	const gifContext = gifCanvas.getContext('2d');

	const renderGif = (frames) => {
		loadedFrames = frames;
		frameIndex = 0;
		console.log(frames[0].dims);
		canvasWidth = frames[0].dims.width;
		canvasHeight = frames[0].dims.height;
		gifCanvas.width = canvasWidth;
		gifCanvas.height = canvasHeight;

		renderFrame();
	};

	const renderFrame = () => {
		const frame = loadedFrames[frameIndex];
		const start = new Date().getTime();

		if (needsDisposal) {
			gifContext?.clearRect(0, 0, canvasWidth, canvasHeight);
			needsDisposal = false;
		}

		drawPatch(frame);

		manipulate();

		frameIndex++;
		if (frameIndex >= loadedFrames.length) {
			frameIndex = 0;
		}

		if (frame.disposalType === 2) {
			needsDisposal = true;
		}
		const end = new Date().getTime();
		const diff = end - start;

		setTimeout(() => {
			requestAnimationFrame(renderFrame);
		}, 1000 / $fps);
	};

	const drawPatch = (frame) => {
		const dims = frame.dims;
		if (
			!frameImageData ||
			dims.width != frameImageData.width ||
			dims.height != frameImageData.height
		) {
			tempCanvas.width = dims.width;
			tempCanvas.height = dims.height;
			frameImageData = tempContext?.createImageData(dims.width, dims.height);
		}

		frameImageData.data.set(frame.patch);

		tempContext?.putImageData(frameImageData, 0, 0);

		gifContext?.drawImage(tempCanvas, dims.left, dims.top);
	};

	const manipulate = () => {
		const imageData = gifContext?.getImageData(0, 0, gifCanvas.width, gifCanvas.height);
		const other = gifContext?.createImageData(gifCanvas.width, gifCanvas.height);
		canvasContext.putImageData(imageData, 0, 0);
	};
</script>

<canvas id="AnimationPreview" height={canvasHeight} width={canvasWidth} />

<style>
	#AnimationPreview {
		height: 64px;
		width: 128px;
	}
</style>
