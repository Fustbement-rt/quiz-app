import { writable } from 'svelte/store';
import { API_URL, get } from '$lib/api';
import { browser } from '$app/environment';

export const token = writable(browser ? localStorage.getItem('token') : null);
export const loginDetails = writable(null);

if (browser) {
	setTimeout(() => {
		token.subscribe(async () => {
			const UserApiUrl = new URL('user_info', API_URL);
			let user_data = await get(UserApiUrl);
			if (user_data.ok) {
				let user_json = await user_data.json();
				loginDetails.set(user_json);
			} else {
				localStorage.removeItem('token');
			}
		});
	}, 100);
}
