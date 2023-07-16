<script>
  import {goto} from "$app/navigation";
  import {invoke} from "@tauri-apps/api/tauri";
  import {onMount} from "svelte";

  onMount(async () => {
    let result = await invoke("initial_check");
    switch (result) {
      case "send-to-app":
        goto("/app");
        break;
      case "send-to-login":
        goto("/login");
        break;
      default:
        console.log("Unable to decode sent by intial_check function.");
    }
  });
</script>

<p>Loading....</p>
