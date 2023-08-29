import { writable } from "svelte/store";

export const gifPath = writable()
export const tempPath = writable()
export const outputPath = writable()
export const flipperzeroDir = writable();
export const gifFrameLength = writable()

// GIF Info
export const animationName = writable('');
export const fps = writable(1);
export const duration = writable()
export const cooldown = writable()

// Manifest Info
export const minButthurt = writable()
export const maxButthurt = writable()
export const minLevel = writable()
export const maxLevel = writable()
export const weight = writable()

// Bubble Info
export const bubbleTextPresent = writable()
export const textBoxX = writable()
export const textBoxY = writable()
export const bubbleText = writable()
export const alignH = writable()
export const alignV = writable()
export const startFrame = writable()
export const endFrame = writable()