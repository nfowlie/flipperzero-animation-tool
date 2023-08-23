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
</script>

<form>
	<label for="animation-name">Animation Name</label>
	<input
		name="animation-name"
		type="text"
		placeholder="Animation Name"
		bind:value={$animationName}
	/>
	<label for="fps">FPS: {$fps}</label>
	<input name="fps" type="range" min="1" max="7" placeholder="FPS" bind:value={$fps} />
	<label for="duration">Duration</label>
	<input
		name="duration"
		type="number"
		step="1"
		placeholder="Duration in Seconds"
		min="1"
		bind:value={$duration}
	/>
	<label for="cooldown">Cooldown</label>
	<input
		name="cooldown"
		type="number"
		step="1"
		placeholder="Cooldown in Seconds"
		min="0"
		bind:value={$cooldown}
	/>
	<label for="min-butthurt">Min butthurt</label>
	<input name="min-butthurt" type="number" placeholder="Min Butthurt" bind:value={$minButthurt} />
	<label for="max-butthurt">Max butthurt</label>
	<input name="max-butthurt" type="number" placeholder="Max Butthurt" bind:value={$maxButthurt} />
	<label for="min-level">Min level</label>
	<input name="min-level" type="number" placeholder="Min Level" bind:value={$minLevel} />
	<label for="max-level">Max level</label>
	<input name="max-level" type="number" placeholder="Max Level" bind:value={$maxLevel} />
	<label for="weight">Weight</label>
	<input name="weight" type="number" placeholder="Weight" bind:value={$weight} />
</form>
