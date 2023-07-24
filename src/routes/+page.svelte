<script lang="ts">
  import {goto} from "$app/navigation";
  import {invoke} from "@tauri-apps/api/tauri";
  import {onMount} from "svelte";
  import {set_current_user_account} from "../stores/user_account_store";
  import {MessageType, push_new_message} from "../stores/toast_store";
  import type {Config} from "../utils/models";
  import {set_current_config} from "../stores/config_store";

  onMount(async () => {
    const result = await invoke("initial_check");
    if (result == "send-to-login") {
      goto("/login");
      return;
    }
    if (result != "send-to-app") {
      push_new_message(
        MessageType.error,
        "Got unexpected result from initial_check function. The app will never load. Open an issue."
      );
      return;
    }

    const config: Config = await invoke("get_config");
    set_current_config(config);
    set_current_user_account(config.user_accounts.find((s) => s.id == config.active_user_account_id)!);
    goto("/app");
  });
</script>

<p>Loading Part1....</p>
