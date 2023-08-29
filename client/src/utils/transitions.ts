import { backOut } from "svelte/easing";
import type { TransitionConfig } from "svelte/transition";

export function fadeScale(node: Element, { duration }: TransitionConfig): TransitionConfig {
	// Need to get the starting values because they could be something other than 1
	const computedStyles = getComputedStyle(node);
	const initOpacity = +computedStyles.opacity;
	// The falsy operator is for if scale isn't set
	const initScale = +computedStyles.scale || 1;
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
