import { browser } from "$app/env"
import { writable } from "svelte/store"

export const auth = writable((browser && localStorage.getItem("auth")) || "")
auth.subscribe(val => {
  if (browser) {
    localStorage.setItem("auth", val)
  }
})
