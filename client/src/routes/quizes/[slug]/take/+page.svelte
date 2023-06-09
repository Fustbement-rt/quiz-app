<script lang="ts">
	import { Title, Subtitle, Content } from '@smui/paper';
	import Button from '@smui/button';
	import Card, { Content as CContent } from '@smui/card';
	import Radio from '@smui/radio';
	import { Label } from '@smui/common';
	import { shuffle } from '$lib/util';
	import FormField from '@smui/form-field';
	export let data;

	let questions: {
		answers: { name: string; right: boolean }[];
		question: string;
	}[] = [];
	for (const question of data.questions) {
		const answers: { name: string; right: boolean }[] = [
			{ name: question.correct, right: true },
			...question.wrong.map((x: string) => {
				return { name: x, right: false };
			}),
		];
		shuffle(answers);
		questions.push({
			question: question.question,
			answers: answers,
		});
	}

	let answers = Array(questions.length);
	let res: null | number = null;

	shuffle(questions);

	const rate = () => {
		let i = 0;
		answers.forEach((answer) => {
			if (answer === true) i++;
		});
		res = Math.round((i * 100) / answers.length);
	};
</script>

<Title>{data.name}</Title>
<Subtitle>Készítette: {data.username}</Subtitle>
<Content>
	{#if res === null}
		<div class="m g">
			{#each questions as question, i}
				<Card>
					<CContent>
						<Label><b>{question.question}</b></Label>
						<div class="g">
							{#each question.answers as answer, j}
								<FormField>
									<Radio
										bind:group={answers[i]}
										value={answer.right ? true : j}
										touch
									/>
									<Label slot="label">{answer.name}</Label>
								</FormField>
							{/each}
						</div>
					</CContent>
				</Card>
			{/each}
		</div>
		<Button color="primary" variant="raised" on:click={rate}>
			<Label>Kész!</Label>
		</Button>
	{:else}
		<p>
			Az eredményed: {res}%.
		</p>
		<Button color="primary" variant="raised" href={`/quizes/${data.slug}`}>
			<Label>Vissza</Label>
		</Button>
	{/if}
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
</style>
