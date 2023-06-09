<script>
	import Paper, { Title, Subtitle, Content } from '@smui/paper';
	import Button from '@smui/button';
	import { Label } from '@smui/common';
	export let data;
</script>

<Title>{data.name}</Title>
<Subtitle>Készítette: {data.username}</Subtitle>
<Content>
	<p>Gyakorláshoz:</p>
	<Button href={`/quizes/${data.id}/flashcards`}>Flash cardok</Button>
	<p>Ellenőrzéshez:</p>
	<Button href={`/quizes/${data.id}/take`}>Kérdések</Button>
</Content>
