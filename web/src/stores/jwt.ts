// import { persist, sessionStorage } from "@macfja/svelte-persistent-store"
import { browser } from "$app/env"
import { writable } from "svelte/store"

export const auth = writable((browser && localStorage.getItem("auth")) || "hestebøf")
auth.subscribe(val => {
  if (browser) {
    localStorage.setItem("auth", val)
  }
})
