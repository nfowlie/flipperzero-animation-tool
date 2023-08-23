<script>
	import { animationName, fps, duration, cooldown } from '../stores';
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
</form>
