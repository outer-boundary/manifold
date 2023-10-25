import { writable } from "svelte/store";

export interface User {
	userID: string,
  displayName: string,
  firstName: string,
  lastName: string,
  dateOfBirth: string,
}

export const currentUserID = writable<string | undefined>(undefined);
