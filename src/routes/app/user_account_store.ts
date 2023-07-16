import {writable} from "svelte/store";
import type {UserAccount} from "../../utils/models";

export const current_user_account = writable<UserAccount>(null!);
