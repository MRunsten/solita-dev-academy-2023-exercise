<script lang="ts">

export let sub_url: string;
export let current_page: number;
export let data_amount: number;
export let data_max_per_page: number;

</script>

<div class="page-links">
    <div>
    {#if current_page > 0}
        <a href="/{sub_url}/{current_page - 1}">Previous page</a>
    {/if}
    </div>

    <div class="right">
        {#if data_amount === data_max_per_page}
            <a href="/{sub_url}/{current_page + 1}">Next page</a>
        {/if}
    </div>
</div>

<style lang="scss">
    @import '../app.scss';

    $button-text-color: $content-background;
    $button-border-radius:8px;

    .page-links {
        width:100%;
        display:flex;
        justify-content: space-between;
        box-sizing: border-box;
    }

    .page-links a {
        display: block;
        padding:12px 16px;
        margin:16px 0;
        background: $color2;
        border-radius: $button-border-radius;
        color: $button-text-color;
        text-decoration: none;
        box-shadow: none;
        
		transition:all 0.1s;
    }

    .page-links a:hover {
        background: $accent;
    }

    @media only screen and (max-width: 880px) {
        .page-links {
            font-size:0.6rem;
        }

        .page-links a {
            margin: 16px;
        }
    }
</style>