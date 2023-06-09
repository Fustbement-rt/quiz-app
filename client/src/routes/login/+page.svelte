<script lang="ts">
	import Button from '@smui/button';
	import TextField from '@smui/textfield';
	import { API_URL, postJSON } from '$lib/api';
	import Paper, { Title, Subtitle, Content } from '@smui/paper';
	import { token } from '$lib/logged';
	import { goto } from '$app/navigation';

	let username: string | null = null;
	let password: string | null = null;
	const handleLogin = async (e: SubmitEvent) => {
		e.preventDefault();
		const login_url = new URL('login', API_URL);
		let res = await postJSON(login_url, {
			username,
			password,
		});
		if (res.ok) {
			let got_token = await res.json();
			token.set(got_token);
			localStorage.setItem('token', got_token);
			goto('/');
		}
	};
</script>

<Title>Bejelentkezés</Title>
<Content>
	<form on:submit={handleLogin}>
		<div>
			<TextField
				type="text"
				bind:value={username}
				label="Felhasználónév"
			/>
		</div>
		<div>
			<TextField type="password" bind:value={password} label="Jelszó" />
		</div>
		<div>
			<Button type="submit" variant="raised">Bejelentkezés</Button>
		</div>
	</form>
</Content>

<style>
	div {
		margin-bottom: 1.5rem;
	}
</style>
