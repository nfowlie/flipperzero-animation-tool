<script>
	import { parseGIF, decompressFrames } from 'gifuct-js';
	import { onMount } from 'svelte';
	import {
		fps,
		textBoxX,
		textBoxY,
		bubbleText,
		alignH,
		alignV,
		startFrame,
		endFrame
	} from '../stores';
	import { text } from '@sveltejs/kit';

	export let gif;
	let loadedFrames,
		frameIndex = 0,
		needsDisposal = false,
		frameImageData,
		canvasHeight = 0,
		canvasWidth = 0,
		canvasPreview,
		canvasContext,
		textOverlay,
		textContext,
		timeout;
	$: if (gif) renderGif(gif);
	$: $fps || $bubbleText, reRender();
	const reRender = () => {
		if (gif) {
			needsDisposal = true;
			renderGif(gif);
		}
	};

	onMount(() => {
		canvasPreview = document.getElementById('AnimationPreview');
		canvasContext = canvasPreview?.getContext('2d');

		textOverlay = document.getElementById('TextOverlay');
		textContext = textOverlay?.getContext('2d');
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
			textContext?.clearRect(0, 0, canvasWidth, canvasHeight);
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
		clearTimeout(timeout);
		timeout = setTimeout(() => {
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
		textContext?.clearRect(0, 0, canvasWidth, canvasHeight);

		if (frameIndex >= $startFrame && frameIndex <= $endFrame) createTextBox();
	};

	const createTextBox = () => {
		textContext.font = '16px haxrcorp-4089';
		let lines = '';
		let textWidth = 0;
		if ($bubbleText) {
			lines = $bubbleText.split('\n');
			lines.forEach((line, index) => {
				let lineStartHeight = 10;
				textContext.fillText(line, $textBoxX + 3, $textBoxY + lineStartHeight + index * 10);
				if (textWidth < textContext.measureText(line).width)
					textWidth = textContext.measureText(line).width;
			});
		}
		canvasContext.beginPath();
		canvasContext.moveTo($textBoxX, $textBoxY);
		if ($alignH === 'Center' && $alignV === 'Top') {
			canvasContext.lineTo($textBoxX + textWidth / 2 - 2, $textBoxY);
			canvasContext.lineTo($textBoxX + textWidth / 2 + 2, $textBoxY - 4);
			canvasContext.lineTo($textBoxX + textWidth / 2 + 6, $textBoxY);
		}
		canvasContext.lineTo($textBoxX + textWidth + 5, $textBoxY);
		if ($alignH === 'Right' && $alignV === 'Top') {
			canvasContext.lineTo($textBoxX + textWidth + 9, $textBoxY);
			canvasContext.lineTo($textBoxX + textWidth + 5, $textBoxY + 4);
		}
		if ($alignH === 'Right' && $alignV === 'Center') {
			canvasContext.lineTo($textBoxX + textWidth + 5, $textBoxY + (lines.length * 10) / 2 - 2);
			canvasContext.lineTo($textBoxX + textWidth + 9, $textBoxY + (lines.length * 10) / 2 + 2);
			canvasContext.lineTo($textBoxX + textWidth + 5, $textBoxY + (lines.length * 10) / 2 + 6);
		}
		if ($alignH === 'Right' && $alignV === 'Bottom') {
			canvasContext.lineTo($textBoxX + textWidth + 5, $textBoxY + lines.length * 10);
			canvasContext.lineTo($textBoxX + textWidth + 9, $textBoxY + lines.length * 10 + 4);
		}
		canvasContext.lineTo($textBoxX + textWidth + 5, $textBoxY + lines.length * 10 + 4);
		if ($alignH === 'Center' && $alignV === 'Bottom') {
			canvasContext.lineTo($textBoxX + textWidth / 2 - 2, $textBoxY + lines.length * 10 + 4);
			canvasContext.lineTo($textBoxX + textWidth / 2 + 2, $textBoxY + lines.length * 10 + 8);
			canvasContext.lineTo($textBoxX + textWidth / 2 + 6, $textBoxY + lines.length * 10 + 4);
		}
		canvasContext.lineTo($textBoxX, $textBoxY + lines.length * 10 + 4);

		if ($alignH === 'Left' && $alignV === 'Top') {
			canvasContext.lineTo($textBoxX, $textBoxY + 4);
			canvasContext.lineTo($textBoxX - 4, $textBoxY);
		}
		if ($alignH === 'Left' && $alignV === 'Center') {
			canvasContext.lineTo($textBoxX, $textBoxY + (lines.length * 10) / 2 - 2);
			canvasContext.lineTo($textBoxX - 4, $textBoxY + (lines.length * 10) / 2 + 2);
			canvasContext.lineTo($textBoxX, $textBoxY + (lines.length * 10) / 2 + 6);
		}
		if ($alignH === 'Left' && $alignV === 'Bottom') {
			canvasContext.lineTo($textBoxX - 4, $textBoxY + lines.length * 10 + 4);
			canvasContext.lineTo($textBoxX, $textBoxY + lines.length * 10);
		}
		canvasContext.lineTo($textBoxX, $textBoxY);

		canvasContext.stroke();
		canvasContext.fillStyle = '#fff';
		canvasContext.fill();
	};
</script>

<div id="AnimationContainer">
	<canvas id="AnimationPreview" height={canvasHeight} width={canvasWidth} />
	<canvas id="TextOverlay" height={canvasHeight} width={canvasWidth} />
</div>

<style>
	#AnimationContainer {
		position: relative;
	}
	#AnimationPreview,
	#TextOverlay {
		height: calc(64px);
		width: calc(128px);
		position: absolute;
		left: 0;
		top: 0;
	}
</style>
