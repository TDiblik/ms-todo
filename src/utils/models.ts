/* ----------- Related to communication between rust and node ----------- */
export interface CommandResult<T> {
  success: boolean;
  err_message: string | null;
  result: T | null;
}

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

/* ----------- Generic Microsoft graph API objects ----------- */
export enum WellknownListName {
  None = "none",
  DefaultList = "defaultList",
  FlaggedEmails = "flaggedEmails",
  UnknownFutureValue = "unknownFutureValue",
}

export enum TaskStatus {
  NotStarted = "notStarted",
  InProgress = "inProgress",
  Completed = "completed",
  WaitingOnOthers = "waitingOnOthers",
  Deferred = "deferred",
}

export enum TaskImportance {
  Low = "low",
  Normal = "normal",
  High = "High",
}

export interface DateTimeTimeZoneGraphObj {
  dateTime: string;
  timeZone: string;
}

/* ----------- Related to Microsoft graph API ----------- */
export interface TaskList {
  id: string;
  displayName: string;
  wellknownListName: WellknownListName;
}

// TODO: add `recurrence` https://learn.microsoft.com/en-us/graph/api/resources/todotask?view=graph-rest-1.0#properties
export interface Task {
  id: string;
  status: TaskStatus;
  importance: TaskImportance;
  categories: string[];
  title: string;
  body: {
    content: string;
    contentType: string;
  };
  bodyLastModifiedDateTime: Date | null;

  createdDateTime: DateTimeTimeZoneGraphObj | null;
  startDateTime: DateTimeTimeZoneGraphObj | null;
  completedDateTime: DateTimeTimeZoneGraphObj | null;
  dueDateTime: DateTimeTimeZoneGraphObj | null;

  hasAttachments: boolean;
  isReminderOn: boolean;
  lastModifiedDateTime: Date | null;
  reminderDateTime: DateTimeTimeZoneGraphObj | null;
}
