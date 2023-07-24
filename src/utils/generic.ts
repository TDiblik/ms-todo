import type {Writable} from "svelte/store";
import {fetch_tasks_lists, task_lists} from "../stores/tasks_lists_store";
import {fetch_tasks_by_list_ids} from "../stores/tasks_store";
import {MessageType, push_new_message} from "../stores/toast_store";
import type {TaskList} from "./models";

// Ensure compile-time correctness when working with dynamic properties
export function nameof<T>(name: keyof T) {
  return name;
}

// Efficient way to count elements, when needed (use inside loops and stuff, where perf matters)
export function count_elements_equal(arr: any[], member: string, desired_value: any) {
  let count = 0;
  for (let i = 0; i < arr.length; i++) {
    if (arr[i][member] == desired_value) {
      count++;
    }
  }
  return count;
}
export function count_elements_not_equal(arr: any[], member: string, desired_value: any) {
  let count = 0;
  for (let i = 0; i < arr.length; i++) {
    if (arr[i][member] != desired_value) {
      count++;
    }
  }
  return count;
}

export function read_store_value_outside_svelte<T>(store: Writable<T>): T {
  let currentValue: T;

  const unsubscribe = store.subscribe((value: T) => {
    currentValue = value;
  });
  unsubscribe();

  return currentValue!;
}

export async function refresh_local_cache() {
  push_new_message(MessageType.info, "Refreshing local cache....");

  await fetch_tasks_lists(true);

  let current_tasks_lists = read_store_value_outside_svelte(task_lists);
  await fetch_tasks_by_list_ids(
    current_tasks_lists.map((s) => s.id),
    true
  );

  push_new_message(MessageType.info, "Local cache refreshed.");
}
