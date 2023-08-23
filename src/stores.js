import { writable } from "svelte/store";

export const gifPath = writable()
export const outputPath = writable()
export const flipperzeroDir = writable();

// GIF Info
export const animationName = writable('');
export const fps = writable(1);
export const duration = writable()
export const cooldown = writable()