<script lang="ts">
    import type { PageData } from './$types';

    export let data: PageData;
</script>

<h1>Station ({data.station_id})</h1>

<table>
    <tr>
        <td>Name</td>
        <td>{data.name.finnish} | {data.name.swedish} | {data.name.english}</td> 
    </tr>
    
    <tr><!-- Todo add check for city name with a single whitespace " " -->
        <td>City</td>
        <td>{data.city.finnish} | {data.city.swedish}</td>
    </tr>
   
    <tr>
        <td>Address</td>
        <td>{data.address.finnish} | {data.address.swedish}</td>
    </tr>
</table>

<br>

<table>
    <tr>
        <td>Total ending journeys to this station</td>
        <td>{data.total_ending_journeys}</td>
    </tr>

    <tr>
        <td>Total starting journeys from this station</td>
        <td>{data.total_starting_journeys}</td>
    </tr>
</table>