import {writable} from "svelte/store";
import type {UserAccount} from "../utils/models";

// In release version, the following code doesn't really matter, since the workflow loads data from config file every time the it starts
// and overwrites the localstorage, however during development, when hot-reloading, svelte stores get wiped, and I needed a way
// to persist the state during development.
export const current_user_account = writable<UserAccount>(
  JSON.parse(localStorage.getItem("current_user_account_temp") ?? "{}")
);

export function set_current_user_account(user_account: UserAccount) {
  localStorage.setItem("current_user_account_temp", JSON.stringify(user_account));
  current_user_account.set(user_account);
}
