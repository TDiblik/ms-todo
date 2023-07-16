<script lang="ts">
  export let full_name: string;
</script>

<button class="btn h-14 mb-2 mr-auto ml-auto w-full md:w-auto flex">
  <img class="h-8 rounded-full" src="/microsoft-icon.svg" alt="Microsoft logo" />
  <p class="text-base font-medium ml-3">{full_name}</p>
</button>
