import { token as TokenS } from './logged';
import { get as getS } from 'svelte/store';

export const API_URL = new URL('https://quiz.vadasz.xyz/api/');

function getHeaders(): Record<string, string> {
	let obj: Record<string, string> = {};
	let token = getS(TokenS);
	if (token) obj['Authorization'] = `Bearer ${token}`;
	return obj;
}

export function postJSON(url: URL, data: Object) {
	return fetch(url, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			...getHeaders(),
		},
		body: JSON.stringify(data),
	});
}

export function get(url: URL) {
	return fetch(url, {
		method: 'GET',
		headers: {
			...getHeaders(),
		},
	});
}
