import { API_URL, get } from '$lib/api';

export async function load({ params }) {
	const quizesURL = new URL('quizes', API_URL);
	let res = await get(quizesURL);
	if (res.ok) {
		let json = await res.json();
		return json;
	}
}
