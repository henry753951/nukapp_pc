<template>
	<div grid place-items-center px-6 py-24 sm="py-32" lg="px-8">
		<div text-center>
			<p text-xl text-emerald-500 font-semibold>
				404
			</p>
			<h1 mt-4 text-3xl font-bold tracking-tight sm="text-5xl">
				{{ $t('error.page-not-found.title') }}
			</h1>
			<p mt-6 text-base text-neutral-800 leading-7>
				{{ $t('error.page-not-found.message') }}
			</p>
			<div mt-10 flex items-center justify-center gap-x-6>
				<Hyperlink to="/" block>
					{{ $t('actions.go-home') }}
				</Hyperlink>
			</div>
		</div>
	</div>
</template>

<script lang="ts" setup>
	definePageMeta({
		name: 'PageNotFound',
	});
</script>
