import {writable} from "svelte/store";
import type {Config} from "../utils/models";

// In release version, the following code doesn't really matter, since the workflow loads data from config file every time the it starts
// and overwrites the localstorage, however during development, when hot-reloading, svelte stores get wiped, and I needed a way
// to persist the state during development.
export const current_config = writable<Config>(JSON.parse(localStorage.getItem("current_user_account_temp") ?? "{}"));

export function set_current_config(config: Config) {
  localStorage.setItem("current_config_temp", JSON.stringify(config));
  current_config.set(config);
}
