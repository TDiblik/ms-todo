<script lang="ts">
  import {open} from "@tauri-apps/api/shell";
  import {onMount} from "svelte";
  import {invoke} from "@tauri-apps/api/tauri";
  import {listen, once} from "@tauri-apps/api/event";
  import {goto} from "$app/navigation";
  import type {Config} from "../../utils/models";
  import {MessageType, push_new_message} from "../toast_store";

  let login_url = "";
  let config: Config | null = null;
  onMount(async () => {
    login_url = await invoke("get_login_url");
    config = await invoke("get_config");
    await invoke("logout");
  });

  async function login() {
    await open(login_url);
  }
  listen("app://login-request-error", (e) => {
    console.log("Errored while logging in.");
    console.log(e);
    let msg = ((e.payload as any)?.msg as any) ?? "Also, unable to retrieve msg from payload.";
    push_new_message(MessageType.error, "Unable to login :( - " + msg);
  });
  once("app://login-request-success", () => {
    goto("/"); // send to "/", just to make sure and run all checks
  });

  async function login_manual(user_id: string) {
    await invoke("login_manual", {userId: user_id});
    goto("/"); // send to "/", just to make sure and run all checks
  }
</script>

<div class="h-full flex bg-gradient-to-r from-sky-500 to-indigo-500 text-slate-200">
  <div
    class="h-4/6 w-9/12 md:w-1/2 md:h-1/2 m-auto border-primary rounded-md bg-slate-900 bg-opacity-80 primary flex flex-col p-2 items-center backdrop-blur shadow-lg"
  >
    <div class="w-10/12">
      <h1 class="text-center mt-3 mb-2 text-xl text-slate-200">Add a new account?</h1>
      <button
        aria-label="Login with Microsoft Account"
        class="btn mr-auto ml-auto w-full md:w-auto flex"
        on:click={login}
      >
        <img src="/microsoft-icon.svg" alt="Microsoft logo" />
        <p class="text-base font-medium ml-3 text-slate-300">Login with Microsoft</p>
      </button>

      <h1 class="text-center mt-4 mb-2 text-xl">Existing accounts</h1>
      {#each config?.user_accounts ?? [] as user_account}
        <button
          class="btn h-16 mb-2 mr-auto ml-auto w-full md:w-auto flex"
          title={user_account.id}
          on:click={() => login_manual(user_account.id)}
        >
          <img class="h-8 rounded-full" src={user_account.profile_photo} alt="Account's profile pic" />
          <div class="ml-3">
            <p class="text-base font-medium text-slate-300">{user_account.display_name}</p>
            <p class="text-sm font-thin text-slate-300 lowercase">{user_account.mail}</p>
          </div>
        </button>
      {/each}
    </div>
  </div>
</div>
