<script lang="ts">
    export let userName: String;
    export let message: String;
    export let time: Date;
    export let is_in: Boolean = true;

    $: time_formatted = time.toLocaleTimeString("en-GB", { hour: "2-digit", minute: "2-digit" });

</script>


<div class="bubbles-group">
    <div class:is-in={is_in} class:is-out={!is_in} class:hide-name={!is_in} class="bubble can-have-tail is-group-first is-group-last" >
        <div class="bubble-content-wrapper">
            <div class="bubble-content">
                <div class="message spoilers-container" dir="auto">
                    {message}
                    <span class="time tgico">
                        <span class="i18n" dir="auto">{time_formatted}</span>
                        <div class="inner tgico">
                            <span class="i18n" dir="auto">{time_formatted}</span>
                        </div>
                    </span>
                </div>
                <div class="colored-name name floating-part" data-peer-id="629594124" style="color: var(--primary-color);">
                    <span class="peer-title" dir="auto" data-peer-id="629594124" data-with-premium-icon="1">
                        {userName}
                    </span></div>
                <svg viewBox="0 0 11 20" width="11" height="20" class="bubble-tail">
                    <use href="#message-tail-filled"></use>
                </svg>
            </div>
        </div>
    </div>
</div>