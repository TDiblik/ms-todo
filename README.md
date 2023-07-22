# WIP

# Dev

## Setup dev env

## Compile for dev

## Compile for production

## Todos (how ironicall to keep them in README xd)

- Implement task search
- Implement ability to change task lists order by dragging (and keep the order in config)
- Implement task groups
- Remove the ugly af native taskbar
  - https://tauri.app/v1/guides/features/window-customization/
  - https://github.com/tauri-apps/tauri/tree/dev/examples/api
- Implement the settings supported by the official Microsoft To Do app
  - Add new tasks on top (on by default)
  - Move starred tasks on top (on by default)
  - Play completion sound (on by default)
  - Confirm before deleting (on by default)
  - Auto-start To Do on setup (will be pain to implement, already implemented once in KeyXpert, but this app won't need admin permissions, so adding path to registry should by enought)
  - Start of the Week (System default <--- default + every day of the week)
  - App Badge (Turned off ; Added to My Day and not completed ; Due today and overdue)
  - Pin to taskbar (windows only, idk if I want to implement this since nobody uses taskbar, because it's full of ads anyways, like wtf, I looked at it right now and somehow I have instagram, messenger, tiktok, and other dumb shit installed, thanks Microsoft, appriciate it <3)
  - Keyboard shortcuts (the official ones are dumb, I want something more vim-like)
  - Theme
    - Dark
    - Light (show popup shaming user for using light mode xdd)
  - Smart lists (todo: plan normal defaults)
    - Important
    - Planned
    - Completed
    - All
    - Assigned to me (uf, don't know about this one, this will probably require access to Tasks.ReadWriteShared or smthng like that and that's probably behind some kind of "require admin access", which could be a huge blocker for a lot of people, just guessing tho)
  - Auto-hide empty smart lists
  - Show "Due today" tasks in "My Day"
  - Tasks assigned to you in Planner (off by default)
  - Forced sync
  - About section
    - Should look like the Microsoft To Do's, but with info about the project and me, without diagnostics switches! (I thought about including them for fun, without actually making them work, howerver some people probably wouldn't get the joke)
- Cleanup TODOs inside code
- Prettify README
  - app screenshots
  - tutorials on how to use
  - dev tutorials
  - how to contribute
  - list of supported features
  - FAQ

## Notes

- Some things are implemented from the ground-up and kept inside a config file. This is 99% of the time, because Microsoft API does not return this information. I could go and reverse engineer how their To Do app works, however not only I don't have the time to do that, but they could just change their (probably internal) API and I would be fucked :). So until they officially implement these things, I'mma roll my own implemention
- I won't implement the "Recognize dates and times in task titles" unless I'm realllly bored someday. Not only does it suck in the official app, but I doubt that I can implement it better than multi-bilion corp
