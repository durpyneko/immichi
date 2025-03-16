import { writable } from "svelte/store";

function createPersistentStore(key: string, initialValue: string) {
  let storedValue = localStorage.getItem(key);
  const store = writable(storedValue ?? initialValue);

  store.subscribe((value) => {
    localStorage.setItem(key, value);
  });

  return store;
}

export const toggleState = createPersistentStore("toggleBgState", "toggle_off");
