<script>
	// @ts-nocheck

	import TopAppBar, { Row, Section, Title } from '@smui/top-app-bar';
	import Button from '@smui/button';
	import { Label } from '@smui/common';
	import { token, loginDetails } from '$lib/logged';

	export const ssr = false;

	function logout() {
		token.set(null);
		loginDetails.set(null);
		localStorage.removeItem('token');
	}
</script>

<TopAppBar variant="static" color="primary" prominent={false} dense={false}>
	<Row>
		<Section>
			<Title>Quiz Experiment</Title>
			<Button href="/"><Label>Főoldal</Label></Button>
			<Button href="/quizes"><Label>Quizek</Label></Button>
		</Section>
		<Section align="end" toolbar>
			{#if $token}
				<Label>
					{#if $loginDetails}
						{$loginDetails.username}
					{:else}
						loading...
					{/if}
				</Label>

				<Button on:click={logout}><Label>Logout</Label></Button>
			{:else}
				<Button href="/login"><Label>Login</Label></Button>
				<Button href="/register"><Label>Register</Label></Button>
			{/if}
		</Section>
	</Row>
</TopAppBar>
