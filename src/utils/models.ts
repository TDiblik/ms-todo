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

export type WellknownListName = "none" | "defaultList" | "flaggedEmails" | "unknownFutureValue";

export interface CommandResult<T> {
  success: boolean;
  err_message: string | null;
  result: T | null;
}

export interface TaskList {
  id: string;
  displayName: string;
  wellknownListName: WellknownListName;
}
