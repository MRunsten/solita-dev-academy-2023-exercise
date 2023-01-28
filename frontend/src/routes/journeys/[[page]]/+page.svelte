<script lang="ts">
    import Pagination from '../../../lib/pagination.svelte';

	import type { PageData } from './$types';
	export let data: PageData;
</script>

<div id="content">
    <div class="large-display">
        <Pagination
            sub_url="journeys"
            current_page={data.page}
            data_amount={data.journeys.length}
            data_max_per_page={data.max_per_page}
        />
    </div>

	<table class="data-list">
		<thead>
			<tr>
				<td><span>Departure</span></td>
				<td />
				<td><span>Return</span></td>
				<td><span>Info</span></td>
			</tr>
		</thead>
		<tbody>
			{#each data.journeys as journey}
				<tr>
					<td width="40%">
                        <a href='/station/{journey.departure_station_id}'>{journey.departure_station_name}</a>
                    </td>
					<td width="2%"><i class="emoji-on-left">➡️</i></td>
					<td width="40%">
                        <a  href='/station/{journey.return_station_id}'>{journey.return_station_name}</a>
                    </td>
					<td width="18%">{journey.distance_kilometers} km</td>
				</tr>

				<tr>
					<td>{journey.departure_date}</td>
					<td />
					<td>{journey.return_date}</td>
					<td>{journey.duration_minutes} min</td>
				</tr>
			{/each}
		</tbody>
	</table>

    <Pagination
    sub_url="journeys"
    current_page={data.page}
    data_amount={data.journeys.length}
    data_max_per_page={data.max_per_page}
    />
</div>

<style lang="scss">
	@import '../../../styles/table.scss';
	@import '../../../styles/emoji.scss';

	#content {
		width: 100%;
		display: flex;
		flex-direction: column;
		padding: 0 16px;
	}

    table.data-list a {
        color:$accent;
        text-decoration: none;
    }

    table.data-list a:hover {
        text-decoration: underline;
    }

	table.data-list > tbody > tr:nth-child(4n-1),
	table.data-list > tbody > tr:nth-child(4n) {
		background: $content-background-alt;
	}

	@media only screen and (max-width: 880px) {
		#content {
			padding: 0;
		}
	}
</style>
