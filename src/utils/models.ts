export interface Config {
  active_user_account_id: string;
  user_accounts: Array<UserAccount>;
}

export interface UserAccount {
  id: string;
  display_name: string;
  mail: string;
  access_token: string;
  access_token_expires_at: Date;
  refresh_token: string;
  profile_photo: string | undefined;
}

export interface CommandResult<T> {
  success: boolean;
  err_message: string | null;
  result: T | null;
}

export enum WellknownListName {
  None = "none",
  DefaultList = "defaultList",
  FlaggedEmails = "flaggedEmails",
  UnknownFutureValue = "unknownFutureValue",
}
export interface TaskList {
  id: string;
  displayName: string;
  wellknownListName: WellknownListName;
}

export enum TaskStatus {
  NotStarted = "notStarted",
  InProgress = "inProgress",
  Completed = "completed",
  WaitingOnOthers = "waitingOnOthers",
  Deferred = "deferred",
}
// TODO: add `recurrence` https://learn.microsoft.com/en-us/graph/api/resources/todotask?view=graph-rest-1.0#properties
export interface Task {
  id: string;
  status: TaskStatus;
  importance: TaskStatus;
  categories: string;
  title: string;
  body: {
    content: string;
    contentType: string;
  };
  bodyLastModifiedDateTime: Date;

  createdDateTime: Date;
  startDateTime: Date;
  completedDateTime: Date;
  dueDateTime: Date;

  hasAttachments: boolean;
  isReminderOn: boolean;
  lastModifiedDateTime: Date;
  reminderDateTime: Date;
}
