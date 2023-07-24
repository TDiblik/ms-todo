<script lang="ts">
  import {onMount} from "svelte";
  import {fetch_tasks_lists, task_lists} from "../../stores/tasks_lists_store";
  import {MessageType, push_new_message} from "../../stores/toast_store";
  import Sidebar from "./Sidebar.svelte";
  import {fetch_tasks_by_list_ids} from "../../stores/tasks_store";
  import {refresh_local_cache} from "../../utils/generic";

  let is_loading = true;
  onMount(async () => {
    // Calls needed for successfull load
    push_new_message(MessageType.info, "Load beginning...");
    await fetch_tasks_lists(false);
    await fetch_tasks_by_list_ids(
      $task_lists.map((s) => s.id),
      false
    );
    is_loading = false;
    push_new_message(MessageType.info, "Finished base load");

    // Calls that can run in the background
    await refresh_local_cache();
  });
</script>

{#if is_loading}
  <p>Loading :) - TODO: Replace with some kind of spinner</p>
{:else}
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
      <button
        on:click={() => {
          fetch_tasks_lists(true);
        }}>fetch false :)</button
      >
      <button
        on:click={() => {
          fetch_tasks_lists(true);
        }}>fetch true :)</button
      >
      <a href="/login">logout</a>

      <label for="sidebar" class="btn btn-primary drawer-button md:hidden">Open drawer</label>
    </div>
    <div class="drawer-side">
      <label for="sidebar" class="drawer-overlay" />
      <Sidebar />
    </div>
  </div>
{/if}
