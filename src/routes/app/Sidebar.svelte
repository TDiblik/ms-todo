<script>
  import CalendarIcon from "../../utils/CalendarIcon.svelte";
  import HouseIcon from "../../utils/HouseIcon.svelte";
  import ListIcon from "../../utils/ListIcon.svelte";
  import PlusIcon from "../../utils/PlusIcon.svelte";
  import SunIcon from "../../utils/SunIcon.svelte";
  import SidebarItem from "./SidebarItem.svelte";
  import {current_user_account} from "../../stores/user_account_store";

  const base_li_classes = "ml-2 mr-2";
</script>

<ul class="menu p-4 w-70 min-h-full bg-base-200 text-base-content">
  <li>
    <button class="h-14 mb-2 mr-auto ml-auto w-full md:w-auto flex">
      <img class="h-12 rounded-full" src={$current_user_account.profile_photo} alt="Account's profile pic" />
      <div class="ml-2 mr-2">
        <p class="text-base font-medium">{$current_user_account.display_name}</p>
        <p class="text-sm font-thin">{$current_user_account.mail}</p>
      </div>
    </button>
  </li>
  <li class={base_li_classes}>
    <input type="text" placeholder="Search" class="input input-bordered input-sm" />
  </li>

  <ul class="menu bg-base-200 min-w-56 rounded-box pb-0">
    <SidebarItem icon={SunIcon} title="My Day" number_of_tasks={1} on_click={() => {}} />
    <SidebarItem icon={CalendarIcon} title="Planned" number_of_tasks={2} on_click={() => {}} />
    <SidebarItem icon={HouseIcon} title="Tasks" number_of_tasks={3} on_click={() => {}} />
  </ul>

  <!-- Acts as a divider. More details at daisyui docs. -->
  <li />

  <!--TODO: Implement groups, microsoft graph api does not return them atm, so either make my own implementaiton or wait for microsfot to implement it ----- Example groups using DaisyUI: https://daisyui.com/components/menu/#collapsible-submenu -->
  <ul class="menu bg-base-200 min-w-56 rounded-box pt-0 pb-0">
    <SidebarItem icon={ListIcon} title="AAAA" number_of_tasks={4} on_click={() => {}} />
    <SidebarItem icon={ListIcon} title="Things to listen to in free time" number_of_tasks={9} on_click={() => {}} />
  </ul>

  <div class="mt-auto">
    <!-- Acts as a divider. More details at daisyui docs. -->
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
