<script lang="ts">
  import CalendarIcon from "../../utils/CalendarIcon.svelte";
  import HouseIcon from "../../utils/HouseIcon.svelte";
  import ListIcon from "../../utils/ListIcon.svelte";
  import PlusIcon from "../../utils/PlusIcon.svelte";
  import SunIcon from "../../utils/SunIcon.svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import {current_user_account} from "../../stores/user_account_store";
  import {sidebar_task_lists} from "../../stores/tasks_lists_store";
  import {UNKNOW_NUMBER_OF_TASKS, tasks} from "../../stores/tasks_store";
  import {count_elements_not_equal, nameof, refresh_local_cache} from "../../utils/generic";
  import type {Task} from "../../utils/models";
  import SidebarSettingsPopupItem from "./SidebarSettingsPopupItem.svelte";
  import {MessageType, push_new_message} from "../../stores/toast_store";
  import {invoke} from "@tauri-apps/api/tauri";
  import {goto} from "$app/navigation";
  import {current_config} from "../../stores/config_store";
  import SettingsIcon from "../../utils/SettingsIcon.svelte";
  import SynchronizationIcon from "../../utils/SynchronizationIcon.svelte";
  import LogoutIcon from "../../utils/LogoutIcon.svelte";

  const base_li_classes = "ml-2 mr-2";
  const status_property_name = nameof<Task>("status");
</script>

<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
<ul class="menu p-4 w-70 min-h-full bg-base-200 text-base-content">
  <li class="dropdown dorpdown-bottom">
    <button class="h-14 mb-2 mr-auto ml-auto w-full flex">
      <img class="h-12 rounded-full" src={$current_user_account.profile_photo} alt="Account's profile pic" />
      <div class="ml-2 mr-2">
        <p class="text-base font-medium">{$current_user_account.display_name}</p>
        <p class="text-sm font-thin">{$current_user_account.mail}</p>
      </div>
    </button>
    <ul tabindex="0" class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box">
      <SidebarSettingsPopupItem
        title="Settings"
        icon={SettingsIcon}
        on_click={() => {
          push_new_message(MessageType.info, "Comming soon :D");
        }}
      />
      <SidebarSettingsPopupItem
        title="Force sync"
        icon={SynchronizationIcon}
        on_click={() => {
          refresh_local_cache();
        }}
      />
      <SidebarSettingsPopupItem
        title="Logout"
        icon={LogoutIcon}
        on_click={async () => {
          await invoke("logout");
          goto("/login");
        }}
      />
      <li />
      {#each $current_config.user_accounts as user_account}
        <button
          class={"btn h-16 w-full flex " +
            (user_account.id == $current_user_account.id
              ? "bg-primary bg-opacity-30 hover:bg-opacity-50 hover:bg-primary"
              : "")}
          title={user_account.id}
          on:click={async () => {
            await invoke("logout");
            await invoke("login_manual", {userId: user_account.id});
            goto("/"); // send to "/", just to make sure and run all checks
          }}
        >
          <img class="h-9 rounded-full" src={user_account.profile_photo} alt="Account's profile pic" />
          <div class="ml-2 text-left">
            <p class="text-sm font-medium text-slate-300">{user_account.display_name}</p>
            <p class="text-sm font-thin text-slate-300 lowercase">{user_account.mail}</p>
          </div>
        </button>
      {/each}
    </ul>
  </li>
  <li class={base_li_classes}>
    <input type="text" placeholder="Search" class="input input-bordered input-sm" />
  </li>

  <ul class="menu bg-base-200 min-w-56 rounded-box pb-0">
    <SidebarItem icon={SunIcon} title="My Day" number_of_tasks={1} on_click={() => {}} />
    <SidebarItem icon={CalendarIcon} title="Planned" number_of_tasks={2} on_click={() => {}} />
    <SidebarItem icon={HouseIcon} title="Tasks" number_of_tasks={3} on_click={() => {}} />
  </ul>

  <!-- Acts as a divider. More details in daisyui docs. -->
  <li />

  <!-- number_of_tasks={$tasks[task_list.id]?.length ?? UNKNOW_NUMBER_OF_TASKS} -->
  <!--TODO: Implement groups, microsoft graph api does not return them atm, so either make my own implementaiton or wait for microsfot to implement it ----- Example groups using DaisyUI: https://daisyui.com/components/menu/#collapsible-submenu -->
  <ul class="menu bg-base-200 min-w-56 rounded-box pt-0 pb-0">
    {#each $sidebar_task_lists as task_list}
      <SidebarItem
        icon={ListIcon}
        title={task_list.displayName}
        number_of_tasks={$tasks[task_list.id] != null
          ? count_elements_not_equal($tasks[task_list.id], status_property_name, "completed")
          : UNKNOW_NUMBER_OF_TASKS}
        on_click={() => {
          console.log($tasks[task_list.id]);
        }}
      />
    {/each}
  </ul>

  <div class="mt-auto">
    <!-- Acts as a divider. More details in daisyui docs. -->
    <li />

    <li class={base_li_classes}>
      <button class="pl-2">
        <PlusIcon />
        <span class="ml-2">New list</span>
        <!-- TODO: "Add groups icon" after implementing groups -->
      </button>
    </li>
  </div>
</ul>
