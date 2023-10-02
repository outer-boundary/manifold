import { redirect } from '@sveltejs/kit';

export function load({ cookies, url }) {
  const authenticated = cookies.get("id");

  // If an unauthenticated user tries to go to any page besides the below, redirect them to the login page
  if (!authenticated && !["/login", "/signup", "/verify"].some(x => x === url.pathname)) {
    throw redirect(307, "/login");
  }
}