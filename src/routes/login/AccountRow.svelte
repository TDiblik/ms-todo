<script lang="ts">
  export let full_name: string;
  export let profile_pic: string;
  export let on_click: () => {};
</script>

<button class="btn h-14 mb-2 mr-auto ml-auto w-full md:w-auto flex" on:click={() => on_click()}>
  <img class="h-8 rounded-full" src={profile_pic} alt="Account's profile pic" />
  <p class="text-base font-medium ml-3">{full_name}</p>
</button>
