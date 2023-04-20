<script lang="ts">
    import {createEventDispatcher} from "svelte";
    export let placeholder = "Placeholder";
    export let text = "";
    export let hasTail = false;

    const dispatch = createEventDispatcher();

    function click(e: KeyboardEvent){
        if (e.code === "Enter" && e.ctrlKey) {
            dispatch("submit", {text: text});
        }
    }
</script>


    <div class="rows-wrapper chat-input-wrapper">
        {#if hasTail}
            <svg viewBox="0 0 11 20" width="11" height="20" class="bubble-tail">
                <use href="#message-tail-filled"></use>
            </svg>
        {/if}
        <div class="new-message-wrapper">
            <div class="input-message-container">
                <div bind:textContent={text} class="input-message-input i18n scrollable scrollable-y no-scrollbar"
                     contenteditable="true" dir="auto" data-placeholder="{placeholder}" tabindex="-1"
                     spellcheck="false" data-ms-editor="true"
                     style="transition-duration: 0ms; height: 37px;" on:keypress = {click}>
                </div>
                <div contenteditable="true" tabindex="-1"
                     class="input-message-input i18n scrollable scrollable-y no-scrollbar input-field-input-fake"
                     spellcheck="false" data-ms-editor="true"></div>
            </div>
            <button class="btn-icon tgico-scheduled btn-scheduled float hide show" tabindex="-1"></button>
        </div>
    </div>
