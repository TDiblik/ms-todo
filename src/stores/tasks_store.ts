// export const tasks = writable<TaskList[]>([]);

// export async function fetch_tasks_lists() {
//   let new_task_lists = (await invoke("get_task_lists")) as CommandResult<TaskList[]>; // TODO: Type
//   if (!new_task_lists.success) {
//     push_new_message(MessageType.error, new_task_lists.err_message!);
//   }
//   tasks.set(new_task_lists.result!);
// }
