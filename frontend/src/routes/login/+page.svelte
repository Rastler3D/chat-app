<script lang="ts">
    import {authentication} from "../../scripts/localStorage.js";
    import {goto} from '$app/navigation'
    import Input from "../../components/Input.svelte";
    import {BACKEND_URL} from "../../scripts/backend_url";

    let username;
    async function login() {
        let response = await fetch(`/api/chat/signup/${username}`, {
            credentials: 'include',
            method: 'GET',
        });

        authentication.set(await response.json());
        await goto("/");
    }
</script>

<div class="chat-input" style="margin: auto">
    <div class="chat-input-container" style="font-family: Roboto; font-size: 20px; display: flex; flex-direction: column; justify-content: center; align-items: center;">
            <div style="margin-bottom: 10px;">Введите имя пользователя</div>
            <Input placeholder="Имя пользователя" bind:text={username} on:submit={login}></Input>
            <div style="width: 50%; margin-top: 20px ">
                <button class="btn-primary btn-short btn-color-primary" on:click={login}> Войти в чат</button>
            </div>
    </div>
</div>