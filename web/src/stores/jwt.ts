import { browser } from "$app/env"
import { writable } from "svelte/store"

export const auth = writable((browser && localStorage.getItem("auth")) || "")
auth.subscribe(val => {
  if (browser) {
    localStorage.setItem("auth", val)
  }
})

export const mitIdAuth = writable(
  (browser && sessionStorage.getItem("mitIdAuth")) || ""
)
mitIdAuth.subscribe(val => {
  if (browser) {
    sessionStorage.setItem("mitIdAuth", val)
  }
})
