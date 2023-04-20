<script lang="ts">
    import {createEventDispatcher} from "svelte";
    import Input from "./Input.svelte";
    export let text: String;

    let button: HTMLButtonElement;
    const dispatch = createEventDispatcher();

    function submit(){
        dispatch("submit", {text: text});
    }
</script>
<div class="chat-input">
    <div class="chat-input-container">
        <div class="rows-wrapper-wrapper">
            <Input placeholder="Сообщение" bind:text on:submit hasTail={true}></Input>
        </div>
        <div class="btn-send-container">
            <div class="record-ripple"></div>
            <button bind:this={button} on:click={submit} class="btn-icon tgico-send btn-circle btn-send animated-button-icon rp send" tabindex="-1">
                <span class="tgico tgico-send"></span>
                <div class="c-ripple"></div></button>

        </div>
    </div>
</div>