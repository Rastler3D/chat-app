<script lang="ts">
    import {createEventDispatcher} from "svelte";
    export let userName = "";

    const dispatch = createEventDispatcher();
    function logout(e: MouseEvent){
        dispatch("logout");
    }
</script>

<div class="sidebar-header topbar" data-floating="0" style="--utils-width:144px;">
    <div class="chat-info-container">
        <div class="chat-info">
            <div class="person">
                <div class="user-title"><span class="peer-title" dir="auto">{userName}</span>
                </div>
            </div>
        </div>
        <div class="chat-utils">
            <button class="btn-icon tgico-logout" on:click={logout}>
            </button>
        </div>
    </div>
</div>