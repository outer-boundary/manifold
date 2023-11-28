import { writable } from "svelte/store";

export interface Domain {
	id: string,
  displayName: string,
  descriptionText: string,
  iconUrl: string,
  bannerUrl: string,
  public: boolean,
  createdAt: Date,
  updatedAt: Date
}

const domainsStore = writable<Domain[] | undefined>(undefined);

export default domainsStore;