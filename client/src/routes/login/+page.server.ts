import { redirect } from '@sveltejs/kit';

export function load({ cookies }) {
  const authenticated = cookies.get("id");

  if (authenticated) {
    throw redirect(307, "/");
  }
}