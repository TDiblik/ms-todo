import {writable} from "svelte/store";
import {generateUUID} from "../utils/uuid";

export enum MessageType {
  success = "success",
  info = "info",
  warning = "warning",
  error = "error",
}
export interface Message {
  id: string;
  type: MessageType;
  message: string;
}

export const toast_messages = writable([] as Array<Message>);

export function push_new_message(type: MessageType, message: string) {
  toast_messages.update((messages) => [...messages, {id: generateUUID(), type, message}]);
}

export function remove_message(id: string) {
  toast_messages.update((messages) => messages.filter((s) => s.id != id));
}
