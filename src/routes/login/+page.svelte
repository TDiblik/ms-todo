<script lang="ts">
  import AccountRow from "./AccountRow.svelte";
  import {onDestroy, onMount} from "svelte";
  import {invoke} from "@tauri-apps/api/tauri";
  import {listen, once} from "@tauri-apps/api/event";

  let login_url = "";
  onMount(async () => {
    login_url = await invoke("get_login_url");
  });

  let login_window: Window | null = null;
  function login() {
    if (login_window == null) {
      login_window = window.open(login_url, "_blank", "location=yes,height=750,width=750,scrollbars=yes,status=yes");
      window.addEventListener("close", () => (login_window = null));
    }
  }
  listen("app://login-request-error", (e) => {
    console.log("fuck, TODO: handle me");
    console.log(e);
  });
  once("app://login-request-success", () => {
    login_window?.close();
    console.log("success");
  });
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
      <AccountRow full_name="Example 123" />
      <AccountRow full_name="Example 321" />
    </div>
  </div>
</div>
