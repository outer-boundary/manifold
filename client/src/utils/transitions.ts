import { backOut } from "svelte/easing";
import type { TransitionConfig } from "svelte/transition";

export function fadeScale(node: Element, { duration }: TransitionConfig): TransitionConfig {
	// Need to get the starting values because they could be something other than 1
	const initOpacity = +getComputedStyle(node).opacity;
	// The nullish coalescing operator is for if scale isn't set
	const initScale = +getComputedStyle(node).scale || 1;
	return {
		duration,
		easing: backOut,
		css: (t) => {
			return `
        opacity: ${initOpacity * t};
        scale: ${initScale * t};
      `;
		}
	};
}
