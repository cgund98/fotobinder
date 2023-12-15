<div class=" bg-gray-900 h-1 overflow-hidden w-full">
	<div class="progress-bar-value bg-teal-500 w-full h-full"></div>
</div>

<style>
	.progress-bar-value {
		animation: indeterminateAnimation 1.25s infinite ease-out;
		transform-origin: 0% 50%;
	}

	@keyframes indeterminateAnimation {
		0% {
			transform: translateX(0) scaleX(0);
		}
		40% {
			transform: translateX(0) scaleX(0.5);
		}
		100% {
			transform: translateX(100%) scaleX(0.5);
		}
	}
</style>
