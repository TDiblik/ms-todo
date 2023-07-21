import {writable} from "svelte/store";
import type {TaskList} from "../utils/models";
import {invoke} from "@tauri-apps/api/tauri";

export const task_lists = writable<TaskList[]>([]);

export async function fetch_tasks_lists() {
  let new_task_lists = (await invoke("get_task_lists")) as any; // TODO: Type
  task_lists.set(new_task_lists.result);
  console.log(new_task_lists);
}

// export function set_current_user_account(user_account: UserAccount) {
//   localStorage.setItem("current_user_account_temp", JSON.stringify(user_account));
//   current_user_account.set(user_account);
// }
