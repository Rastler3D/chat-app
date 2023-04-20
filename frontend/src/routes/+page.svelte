<script lang="ts">
    import Message from "../components/Message.svelte";
    import ChatInput from "../components/ChatInput.svelte";
    import {onMount, tick} from "svelte";
    import {goto} from "$app/navigation"
    import {authentication} from "../scripts/localStorage";
    import {get} from "svelte/store"
    import Connection from "../components/Connection.svelte";
    import Disconnection from "../components/Disconnection.svelte";
    import UserInfo from "../components/UserInfo.svelte";
    import {BACKEND_URL} from "../scripts/backend_url";

    let user_id: number | undefined = undefined;
    let user_name: string | undefined  = undefined;
    let message: string | null = '';
    let messages: any[] = [];
    let event: EventSource;
    let scroll: HTMLElement;

    async function logout(){
        await fetch("/api/chat/send", {
            credentials: 'include',
            method: 'GET',
        });
        await goto("/login");
    }
    async function send(event) {
        await fetch("/api/chat/send", {
            credentials: 'include',
            method: 'POST',
            body: event.detail.text
        });

        message = null;
        scroll.scrollTo({top: scroll.scrollHeight, behavior: 'smooth'});
    }

    onMount(async () => {
        let response = await fetch("/api/chat/login", {
            credentials: 'include',
            method: 'GET',
        });
        if (response.status === 401) {
            await goto('/login');
        }

        ({ user_id, user_name } = get(authentication));

        let response_history = await fetch("/api/chat/history", {
            credentials: 'include',
            method: 'GET',
        });
        messages = (await response_history.json()).map(data =>[Message, {
            userName: data.user_name,
            message: data.text,
            time: new Date(data.creation_date),
            is_in: user_id !== data.user_id
        }]);

        event = new EventSource("/api/chat", {
            withCredentials: true
        });
        event.addEventListener("Message", event => {
            let data = JSON.parse(event.data);
            messages = [...messages, [Message, {
                userName: data.user_name,
                message: data.text,
                time: new Date(data.creation_date),
                is_in: user_id !== data.user_id
            }]];
        });
        event.addEventListener("Connection", event => {
            let data = JSON.parse(event.data);
            console.log(event);
            messages = [...messages, [Connection, {userName: data.name}]];
        });
        event.addEventListener("Disconnection", event => {
            let data = JSON.parse(event.data);
            messages = [...messages, [Disconnection, {userName: data.name}]];
        });

        await tick();
        scroll.scrollTo({top: scroll.scrollHeight, behavior: 'smooth'});
    })
</script>

<UserInfo userName={user_name} on:logout={logout}></UserInfo>
<div class="bubbles has-groups has-sticky-dates scrolled-down">
    <div bind:this = {scroll}  class="scrollable scrollable-y">
        <div  class="bubbles-inner has-rights">
            <section class="bubbles-date-group">
                {#each messages as [component, props] }
                    <svelte:component this={component} {...props} />
                {/each}
            </section>
        </div>
    </div>
</div>
<ChatInput on:submit={send} bind:text={message}/>
