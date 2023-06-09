<script lang="ts">
	import { API_URL, postJSON } from '$lib/api';
	import Button from '@smui/button';
	import TextField from '@smui/textfield';
	import HelperText from '@smui/textfield/helper-text';
	import Paper, { Title, Subtitle, Content } from '@smui/paper';

	let username = '';
	let password = '';
	let again = '';
	let invalid = true;
	const validate = () => {
		invalid = password != again;
	};

	const handleRegister = (e: SubmitEvent) => {
		e.preventDefault();
		const register_url = new URL('register', API_URL);
		if (invalid) return;
		postJSON(register_url, {
			username,
			password,
		});
	};
</script>

<Title>Regisztráció</Title>
<Content>
	<form on:submit={handleRegister}>
		<div>
			<TextField
				type="text"
				bind:value={username}
				label="Felhasználónév"
			/>
		</div>
		<div>
			<TextField
				type="password"
				bind:value={password}
				label="Jelszó"
				on:blur={validate}
			/>
		</div>
		<div>
			<TextField
				type="password"
				bind:value={again}
				bind:invalid
				on:blur={validate}
				label="Jelszó újra"
			>
				<HelperText validationMsg slot="helper">
					Nem egyeznek a jelszavak
				</HelperText>
			</TextField>
		</div>
		<div>
			<Button type="submit" variant="raised" bind:disabled={invalid}>
				Regisztráció
			</Button>
		</div>
	</form>
</Content>

<style>
	div {
		margin-bottom: 1.5rem;
	}
</style>
