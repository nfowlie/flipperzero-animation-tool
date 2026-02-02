import { writable } from "svelte/store";

export const gifPath = writable()
export const tempPath = writable()
export const outputPath = writable()
export const flipperzeroDir = writable();
export const gifFrameLength = writable()

// GIF Info
export const animationName = writable('');
export const fps = writable(1);
export const duration = writable(3600)
export const cooldown = writable(1)

// Manifest Info
export const minButthurt = writable(0)
export const maxButthurt = writable(3)
export const minLevel = writable(0)
export const maxLevel = writable(3)
export const weight = writable(3)

// Bubble Info
export const bubbleTextPresent = writable()
export const textBoxX = writable()
export const textBoxY = writable()
export const bubbleText = writable()
export const alignH = writable()
export const alignV = writable()
export const startFrame = writable()
export const endFrame = writable()
