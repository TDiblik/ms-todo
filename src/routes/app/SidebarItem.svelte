<script lang="ts">
  import type {ComponentType} from "svelte";
  import {UNKNOW_NUMBER_OF_TASKS} from "../../stores/tasks_store";

  export let icon: ComponentType;
  export let title: string;
  export let number_of_tasks: number;
  export let on_click: () => void;

  const MAX_TITLE_LENGTH = 30;
  $: title = title.length > MAX_TITLE_LENGTH ? (title = title.substring(0, MAX_TITLE_LENGTH) + "...") : title;
</script>

<button on:click={() => on_click()}>
  <li>
    <span class="pl-1">
      <span class="w-6">
        <svelte:component this={icon} />
      </span>
      {title}
      {#if number_of_tasks > 0}
        <span class=" rounded-full bg-slate-900 pr-1 pl-1"> {number_of_tasks} </span>
      {:else if number_of_tasks == UNKNOW_NUMBER_OF_TASKS}
        <span class=" rounded-full bg-slate-900 pr-1 pl-1"> ? </span>
      {/if}
    </span>
  </li>
</button>
