<script>
	import { parseGIF, decompressFrames } from 'gifuct-js';
	import { onMount } from 'svelte';
	import { fps, textBoxX, textBoxY, bubbleText } from '../stores';
	import { text } from '@sveltejs/kit';

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
			if (gif) {
				needsDisposal = true;
				renderGif(gif);
			}
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

		createTextBox();
	};

	const createTextBox = () => {
		canvasContext.font = '16px haxrcorp-4089';
		let lines = '';
		let textWidth = 0;
		if ($bubbleText) {
			lines = $bubbleText.split('\n');
			lines.forEach((line, index) => {
				let lineStartHeight = 13;
				if (index !== 0) lineStartHeight = 0;
				canvasContext.strokeText(line, $textBoxX + 3, $textBoxY + lineStartHeight + index * 16);
				if (textWidth < canvasContext.measureText(line).width)
					textWidth = canvasContext.measureText(line).width;
			});
		}
		canvasContext.beginPath();
		canvasContext.moveTo($textBoxX, $textBoxY);
		canvasContext.lineTo($textBoxX + textWidth + 5, $textBoxY);
		canvasContext.lineTo($textBoxX + textWidth + 5, $textBoxY + lines.length * 10 + 4);
		canvasContext.lineTo($textBoxX, $textBoxY + lines.length * 10 + 4);

		canvasContext.stroke();
		canvasContext.fillStyle = '#fff';
		canvasContext.fill();

		if ($bubbleText) {
			lines.forEach((line, index) => {
				let lineStartHeight = 10;
				// if (index !== 0) lineStartHeight = 0;
				canvasContext.strokeText(line, $textBoxX + 3, $textBoxY + lineStartHeight + index * 10);
			});
		}
	};
</script>

<canvas id="AnimationPreview" height={canvasHeight} width={canvasWidth} />

<style>
	#AnimationPreview {
		height: calc(64px * 2);
		width: calc(128px * 2);
	}
</style>
