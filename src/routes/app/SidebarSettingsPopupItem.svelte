<script lang="ts">
  import type {ComponentType} from "svelte";

  export let icon: ComponentType;
  export let title: string;
  export let on_click: () => void;
</script>

<li>
  <button on:click={() => on_click()}>
    <span class="w-6 text-sm">
      <svelte:component this={icon} />
    </span>
    <button class="text-left">{title}</button>
  </button>
</li>
