<script>
  import {current_user_account} from "./user_account_store";
</script>

<ul class="menu p-4 w-70 h-full bg-base-200 text-base-content">
  <li>
    <button class="h-14 mb-2 mr-auto ml-auto w-full md:w-auto flex">
      <img class="h-12 rounded-full" src={$current_user_account.profile_photo} alt="Account's profile pic" />
      <div class="ml-3">
        <p class="text-base font-medium">{$current_user_account.display_name}</p>
        <p class="text-sm font-thin">{$current_user_account.mail}</p>
      </div>
    </button>
  </li>
</ul>
