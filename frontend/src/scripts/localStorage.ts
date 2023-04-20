import {persisted} from 'svelte-local-storage-store'

export const authentication = persisted('authentication', null as {user_id: number, user_name: string, });