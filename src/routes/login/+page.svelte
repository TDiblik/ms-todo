<script lang="ts">
  import AccountRow from "./AccountRow.svelte";
  import {onDestroy, onMount} from "svelte";
  import {invoke} from "@tauri-apps/api/tauri";
  import {listen, once} from "@tauri-apps/api/event";
  import {goto} from "$app/navigation";
  import type {Config} from "../../utils/models";

  let login_url = "";
  let config: Config | null = null;
  onMount(async () => {
    login_url = await invoke("get_login_url");
    config = await invoke("get_config");
    await invoke("logout");
  });

  let login_window: Window | null = null;
  let auth_did_err = false;
  function login() {
    if (login_window == null) {
      login_window = window.open(login_url, "_blank", "location=yes,height=750,width=750,scrollbars=yes,status=yes");
      window.addEventListener("close", () => (login_window = null));
    }
  }
  listen("app://login-request-error", (e) => {
    console.log("fuck, TODO: handle me");
    console.log(e);
    auth_did_err = true;
  });
  once("app://login-request-success", () => {
    login_window?.close();
    goto("/"); // send to "/", just to make sure and run all checks
  });

  async function login_manual(user_id: string) {
    await invoke("login_manual", {userId: user_id});
    goto("/"); // send to "/", just to make sure and run all checks
  }
</script>

<div class="h-full flex">
  <div
    class="h-4/6 w-9/12 md:w-1/2 md:h-1/2 m-auto border-primary bg-slate-800 rounded-md bg-opacity-95 primary flex flex-col p-2 items-center backdrop-blur shadow-lg"
  >
    <div class="w-10/12">
      <h1 class="text-center mt-3 mb-2 text-xl">Add a new account?</h1>
      <button
        aria-label="Login with Microsoft Account"
        class="btn mr-auto ml-auto w-full md:w-auto flex"
        on:click={login}
      >
        <img src="/microsoft-icon.svg" alt="Microsoft logo" />
        <p class="text-base font-medium ml-3">Login with Microsoft</p>
      </button>

      <h1 class="text-center mt-4 mb-2 text-xl">Existing accounts</h1>
      {#each config?.user_accounts ?? [] as user_account}
        <AccountRow
          full_name={user_account.display_name}
          profile_pic={user_account.profile_photo ?? ""}
          on_click={() => login_manual(user_account.id)}
        />
      {/each}
      {#if auth_did_err}
        <p class="text-center text-red-500 mt-5">Ooops, auth did err, check logs and open issue on github :/</p>
      {/if}
    </div>
  </div>
</div>
