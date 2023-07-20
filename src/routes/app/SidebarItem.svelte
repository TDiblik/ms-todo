<script lang="ts">
  import type {ComponentType} from "svelte";

  export let icon: ComponentType;
  export let title: string;
  export let number_of_tasks: number;
  export let on_click: () => void;

  const MAX_TITLE_LENGTH = 30;
  if (title.length > MAX_TITLE_LENGTH) {
    title = title.substring(0, MAX_TITLE_LENGTH) + "...";
  }
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
      {/if}
    </span>
  </li>
</button>
