<script>
    import { page } from '$app/stores';
  </script>
  

  <h3>{$page?.error?.message}</h3>

  <style>
    h3 {
        width:100%;
        display: flex;
        margin-left:16px;
    }
  </style>