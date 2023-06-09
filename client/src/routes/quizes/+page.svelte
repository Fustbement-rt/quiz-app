<script>
	// @ts-nocheck

	import Card, { Content, ActionButtons, Actions } from '@smui/card';
	import Paper, { Title, Subtitle, Content as PConstent } from '@smui/paper';
	import Button from '@smui/button';
	import { Label } from '@smui/common';
	export let data;
</script>

<Title>Quizek</Title>
<Subtitle>{data.quizes.length} db quiz</Subtitle>
<PConstent>
	<div>
		{#each data.quizes as quiz}
			<Card>
				<Content>
					<p class="mdc-typography--headline6">
						{quiz.name}
					</p>
					<p>Készítette: {quiz.username}</p>
				</Content>
				<Actions>
					<ActionButtons>
						<Button href={`/quizes/${quiz.id}`}>
							<Label>More details</Label>
						</Button>
					</ActionButtons>
				</Actions>
			</Card>
		{/each}
	</div>
</PConstent>

<style>
	div {
		display: grid;
		grid-template-columns: 1fr 1fr 1fr;
		gap: 2rem;
	}
</style>
