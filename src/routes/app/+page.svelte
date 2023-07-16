<script lang="ts">
  import {MessageType, push_new_message} from "../toast_store";
  import Sidebar from "./Sidebar.svelte";
</script>

<div class="drawer md:drawer-open">
  <input id="sidebar" type="checkbox" class="drawer-toggle" />
  <div class="drawer-content flex flex-col items-center justify-center">
    <!-- Page content here -->
    <button
      on:click={() => {
        push_new_message(MessageType.success, "AAA");
      }}>succ</button
    >
    <button
      on:click={() => {
        push_new_message(MessageType.warning, "BBB");
      }}>warning</button
    >
    <button
      on:click={() => {
        push_new_message(MessageType.error, "CCC");
      }}>err</button
    >
    <a href="/login">logout</a>

    <label for="sidebar" class="btn btn-primary drawer-button md:hidden">Open drawer</label>
  </div>
  <div class="drawer-side">
    <label for="sidebar" class="drawer-overlay" />
    <Sidebar />
  </div>
</div>
