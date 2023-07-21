<script lang="ts">
  import {remove_message, toast_messages, type Message} from "../stores/toast_store";

  toast_messages.subscribe((v) => {
    if (v.length != 0) {
      setTimeout(() => {
        remove_message(v[v.length - 1].id);
      }, 7500);
    }
  });
</script>

<div class="toast toast-top toast-end">
  {#each $toast_messages as toast_message}
    <div class={`alert alert-${toast_message.type} text-sm`}>
      <span>{toast_message.message}</span>
      <button
        on:click={() => {
          remove_message(toast_message.id);
        }}
      >
        <svg width="24px" height="24px" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M7 17L16.8995 7.10051" stroke="#000000" stroke-linecap="round" stroke-linejoin="round" />
          <path d="M7 7.00001L16.8995 16.8995" stroke="#000000" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
      </button>
    </div>
  {/each}
</div>

<!-- 
  Ignore this. This just forces tailwind / daisyui / any other tree shaking tool to not strip out following classes from css output (as they are used dynamicall (as they are used dynamically)y). 
  This is a dumb hack, but works ig 
-->
<div class="alert-success" />
<div class="alert-info" />
<div class="alert-warning" />
<div class="alert-error" />
