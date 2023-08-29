<script>
	import {
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
		bubbleText,
		alignH,
		alignV,
		startFrame,
		endFrame,
		gifFrameLength,
		bubbleTextPresent
	} from '../stores';
	import GifConvert from './GIFConvert.svelte';
	const checkIfBubbleText = () => {
		if ($bubbleText?.trim() == '' || $bubbleText == undefined) bubbleTextPresent.set(false);
		else bubbleTextPresent.set(true);
	};
	let gifConvert;
	bubbleTextPresent.set(false);
	$: $bubbleText, checkIfBubbleText();
</script>

<form
	id="gif-form"
	on:submit|preventDefault={() => {
		gifConvert.convertGif();
	}}
>
	<h2>Animation Information</h2>
	<div />
	<div>
		<label for="animation-name">Animation Name</label>
		<input
			required
			name="animation-name"
			type="text"
			placeholder="Animation Name"
			bind:value={$animationName}
		/>
	</div>
	<div>
		<label for="fps">FPS: {$fps}</label>
		<input required name="fps" type="range" min="1" max="7" placeholder="FPS" bind:value={$fps} />
	</div>
	<div>
		<label for="duration">Duration</label>
		<input
			required
			name="duration"
			type="number"
			step="1"
			placeholder="Duration in Seconds"
			min="1"
			bind:value={$duration}
		/>
	</div>
	<div>
		<label for="cooldown">Cooldown</label>
		<input
			required
			name="cooldown"
			type="number"
			step="1"
			placeholder="Cooldown in Seconds"
			min="1"
			bind:value={$cooldown}
		/>
	</div>
	<h2>Animation Settings</h2>
	<div />
	<div>
		<label for="min-butthurt">Min butthurt</label>
		<input
			required
			name="min-butthurt"
			type="number"
			placeholder="Min Butthurt"
			min="0"
			bind:value={$minButthurt}
		/>
	</div>
	<div>
		<label for="max-butthurt">Max butthurt</label>
		<input
			required
			name="max-butthurt"
			type="number"
			placeholder="Max Butthurt"
			min={$minButthurt + 1}
			bind:value={$maxButthurt}
		/>
	</div>
	<div>
		<label for="min-level">Min level</label>
		<input
			required
			name="min-level"
			type="number"
			placeholder="Min Level"
			min="0"
			bind:value={$minLevel}
		/>
	</div>
	<div>
		<label for="max-level">Max level</label>
		<input
			required
			name="max-level"
			type="number"
			placeholder="Max Level"
			min={$minLevel + 1}
			bind:value={$maxLevel}
		/>
	</div>
	<div>
		<label for="weight">Weight</label>
		<input required name="weight" type="number" placeholder="Weight" min="1" bind:value={$weight} />
	</div>
	<div />
	<h2>Bubble Info</h2>
	<div />
	<div>
		<label for="text-box-x">Text Box X</label>
		<input
			required={$bubbleTextPresent}
			name="text-box-x"
			type="number"
			min="0"
			max="128"
			bind:value={$textBoxX}
			placeholder="Text Box X Coordinate"
		/>
	</div>
	<div>
		<label for="text-box-y">Text Box Y</label>
		<input
			required={$bubbleTextPresent}
			name="text-box-y"
			type="number"
			min="0"
			max="64"
			bind:value={$textBoxY}
			placeholder="Text Box Y Coordinate"
		/>
	</div>
	<div>
		<label for="align-h">Align Horizontal</label>
		<input
			id="left"
			type="radio"
			name="align-h"
			required={$bubbleTextPresent}
			bind:group={$alignH}
			value="Left"
		/>
		<label for="left">Left</label>
		<input
			id="center"
			type="radio"
			name="align-h"
			required={$bubbleTextPresent}
			bind:group={$alignH}
			value="Center"
		/>
		<label for="center">Center</label>
		<input
			id="right"
			type="radio"
			name="align-h"
			required={$bubbleTextPresent}
			bind:group={$alignH}
			value="Right"
		/>
		<label for="right">Right</label>
	</div>
	<div>
		<label for="align-v">Align Vertical</label>
		<input
			id="top"
			type="radio"
			name="align-v"
			required={$bubbleTextPresent}
			bind:group={$alignV}
			value="Top"
		/>
		<label for="top">Top</label>
		<input
			id="center"
			type="radio"
			name="align-v"
			required={$bubbleTextPresent}
			bind:group={$alignV}
			value="Center"
		/>
		<label for="center">Center</label>
		<input
			id="bottom"
			type="radio"
			name="align-v"
			required={$bubbleTextPresent}
			bind:group={$alignV}
			value="Bottom"
		/>
		<label for="bottom">Bottom</label>
	</div>
	<div>
		<label for="start-frame">Start Frame</label>
		<input
			required={$bubbleTextPresent}
			name="start-frame"
			type="number"
			min="0"
			max={$gifFrameLength - 1}
			bind:value={$startFrame}
			placeholder="Frame to start bubble"
		/>
	</div>
	<div>
		<label for="end-frame">End Frame</label>
		<input
			required={$bubbleTextPresent}
			name="end-frame"
			type="number"
			min="0"
			max={$gifFrameLength - 1}
			bind:value={$endFrame}
			placeholder="Frame to end bubble"
		/>
	</div>
	<div>
		<label for="bubble-text">Bubble Text</label>
		<textarea name="bubble-text" bind:value={$bubbleText} placeholder="Bubble Text" />
	</div>
</form>

<GifConvert bind:this={gifConvert} />

<style>
	form {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		column-gap: 5%;
		padding-inline: 1.2rem;
	}

	label,
	input,
	textarea {
		font-size: 1.25rem;
	}
	input:not([type='range']),
	textarea {
		background-color: var(--transparency-color);
		border-color: var(--main-color);
		color: var(--main-color);
	}
	input::placeholder,
	textarea::placeholder {
		color: var(--main-color);
	}
	input:focus,
	textarea {
		border-color: var(--main-color);
	}
	form > h2 {
		margin: 0;
	}

	[type='range']:active {
		--range-thumb-color: var(--main-color);
	}
	textarea {
		resize: none;
	}
</style>
