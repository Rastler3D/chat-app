<script lang="ts">
    export let userName: String
</script>

<div class="bubbles-group">
    <div class="bubble service is-group-first is-group-last">
        <div class="bubble-content-wrapper">
            <div class="bubble-content">
                <div class="service-msg">
                    <span class="i18n">
                        Чат покинул пользователь
                        <span class="peer-title" dir="auto" >{userName}</span>
                    </span>
                </div>
            </div>
        </div>
    </div>
</div>