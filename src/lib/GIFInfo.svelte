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
		weight
	} from '../stores';
	import GifConvert from './GIFConvert.svelte';

	let gifConvert;
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
			min="0"
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
			min="0"
			bind:value={$maxLevel}
		/>
	</div>
	<div>
		<label for="weight">Weight</label>
		<input required name="weight" type="number" placeholder="Weight" min="1" bind:value={$weight} />
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
	input {
		font-size: 1.25rem;
	}
	input:not([type='range']) {
		background-color: var(--transparency-color);
		border-color: var(--main-color);
		color: var(--main-color);
	}
	input::placeholder {
		color: var(--main-color);
	}
	input:focus {
		border-color: var(--main-color);
	}
	form > h2 {
		margin: 0;
	}

	[type='range']:active {
		--range-thumb-color: var(--main-color);
	}
</style>
