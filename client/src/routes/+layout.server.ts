import { redirect } from '@sveltejs/kit';

export function load({ cookies, url }) {
  const authenticated = cookies.get("id");

  if (!authenticated && !["/login", "/signup", "/verify"].some(x => x === url.pathname)) {
    throw redirect(307, "/login");
  }
}