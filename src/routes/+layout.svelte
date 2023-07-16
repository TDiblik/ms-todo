<script>
  import "../app.css";
  import ToastMessages from "./ToastMessages.svelte";
</script>

<!-- Order matters! -->
<slot />
<ToastMessages />
