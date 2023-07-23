import {writable} from "svelte/store";
import type {CommandResult, Task} from "../utils/models";
import {invoke} from "@tauri-apps/api";
import {MessageType, push_new_message} from "./toast_store";

export const UNKNOW_NUMBER_OF_TASKS = -9999;
export type TasksMap = {[list_id: string]: Task[]};

export const tasks = writable<TasksMap>({});

export async function fetch_tasks_by_list_ids(ids: string[], refresh_cache: boolean) {
  const function_to_call = refresh_cache ? "get_tasks_by_list_ids" : "get_tasks_by_list_ids_cached";
  const new_task_lists = (await invoke(function_to_call, {ids: ids})) as any; //TODO: Type
  if (!new_task_lists.success) {
    push_new_message(MessageType.error, new_task_lists.err_message!);
  }
  tasks.set(new_task_lists.result!);
}
