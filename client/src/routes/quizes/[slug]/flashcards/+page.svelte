<script lang="ts">
	import { Title, Subtitle, Content } from '@smui/paper';
	import Button from '@smui/button';
	import Card, { Content as CContent } from '@smui/card';
	import { Label } from '@smui/common';
	export let data;
</script>

<Title>{data.name}</Title>
<Subtitle>Készítette: {data.username}</Subtitle>
<Content>
	<div class="m g">
		{#each data.flashcards as card}
			<Card>
				<CContent>
					<Label><b>{card.front}</b></Label>
					<p><i>Mutasson az egérrel a felfedéshez</i></p>
					<p class="h">{card.back}</p>
				</CContent>
			</Card>
		{/each}
	</div>
	<Button color="primary" variant="raised" href={`/quizes/${data.slug}`}>
		<Label>Vissza</Label>
	</Button>
</Content>

<style>
	.m {
		gap: 2ch;
		margin: 2ch;
	}

	.g {
		display: grid;
		grid-template-columns: 1fr 1fr;
	}

	.h {
		background-color: #000;
		color: #000;
	}

	.h:hover {
		background-color: unset;
		color: unset;
	}
</style>
