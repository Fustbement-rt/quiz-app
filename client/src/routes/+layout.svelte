<script>
	import Nav from '$lib/nav.svelte';
	import Footer from '$lib/footer.svelte';
	import '$lib/common.scss';
	import Paper, { Title, Subtitle, Content } from '@smui/paper';
</script>

<div style="min-height: 100vh;">
	<Nav />
	<Paper variant="unelevated">
		<slot />
	</Paper>
</div>
<Footer />
