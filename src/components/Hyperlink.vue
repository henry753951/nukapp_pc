<template>
  <NuxtLink
    v-if="!block"
    transition-colors
    gap-1
    rounded-md
    font-semibold
    text-blue-700
    focus-visible="outline-2 outline-emerald-600 outline-offset-2 outline"
  >
    <slot />
  </NuxtLink>
  <NuxtLink
	v-else
	transition-colors
	gap-1
	rounded-md
	font-semibold
	py-1 px-2
	bg-blue-100
	text-blue-700
	focus-visible="outline-2 outline-emerald-600 outline-offset-2 outline"
  >
	<slot />
  </NuxtLink>
</template>

<script setup lang="ts">
const props = defineProps({
  block: {
    type: Boolean,
    default: false,
  },
});
</script>
