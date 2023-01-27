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
    .page-links {
        width:100%;
        display:flex;
        justify-content: space-between;
        box-sizing: border-box;
    }

    .page-links a {
        display: block;
        padding:16px;
    }

    @media only screen and (max-width: 880px) {
        .page-links {
            font-size:0.6rem;
        }
    }
</style>