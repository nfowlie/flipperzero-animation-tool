<script lang="ts">
	import { run, self, createBubbler, stopPropagation } from 'svelte/legacy';

	const bubble = createBubbler();
	import { open } from '@tauri-apps/plugin-dialog';

	let { showLoading = $bindable() } = $props();

	let dialog = $state();

	run(() => {
		if (dialog && showLoading) dialog.showModal();
	});
	run(() => {
		if (dialog && !showLoading) dialog.close();
	});
</script>

<dialog
	bind:this={dialog}
	onclose={() => (showLoading = false)}
	onclick={self(() => dialog.close())}
>
	<div class="center" onclick={stopPropagation(bubble('click'))}>
		<div class="wave"></div>
		<div class="wave"></div>
		<div class="wave"></div>
		<div class="wave"></div>
		<div class="wave"></div>
		<div class="wave"></div>
		<div class="wave"></div>
		<div class="wave"></div>
		<div class="wave"></div>
		<div class="wave"></div>
	</div>
</dialog>

<style>
	.center {
		height: 100vh;
		display: flex;
		justify-content: center;
		align-items: center;
	}
	.wave {
		width: 5px;
		height: 100px;
		background: linear-gradient(45deg, var(--main-color), var(--transparency-color));
		margin: 10px;
		animation: wave 1s linear infinite;
		border-radius: 20px;
	}
	.wave:nth-child(2) {
		animation-delay: 0.1s;
	}
	.wave:nth-child(3) {
		animation-delay: 0.2s;
	}
	.wave:nth-child(4) {
		animation-delay: 0.3s;
	}
	.wave:nth-child(5) {
		animation-delay: 0.4s;
	}
	.wave:nth-child(6) {
		animation-delay: 0.5s;
	}
	.wave:nth-child(7) {
		animation-delay: 0.6s;
	}
	.wave:nth-child(8) {
		animation-delay: 0.7s;
	}
	.wave:nth-child(9) {
		animation-delay: 0.8s;
	}
	.wave:nth-child(10) {
		animation-delay: 0.9s;
	}

	@keyframes wave {
		0% {
			transform: scale(0);
		}
		50% {
			transform: scale(1);
		}
		100% {
			transform: scale(0);
		}
	}
</style>
