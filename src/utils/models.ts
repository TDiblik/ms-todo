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
