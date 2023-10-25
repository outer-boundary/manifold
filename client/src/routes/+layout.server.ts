import { redirect } from '@sveltejs/kit';
import { currentUserID } from '../stores/currentUser.js';

export function load({ cookies, url }) {
  const authenticated = cookies.get("id");

  // If an unauthenticated user tries to go to any page besides the below, redirect them to the login page
  if (!authenticated && !["/login", "/signup", "/verify"].some(x => x === url.pathname)) {
    throw redirect(307, "/login");
  } else {
    console.log(cookies.get("user_id"))
    currentUserID.set(cookies.get("user_id"));
  }
}