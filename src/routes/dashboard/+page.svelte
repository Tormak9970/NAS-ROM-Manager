<script lang="ts">
    import { PathField } from "@interactables";
    import { FileSelectionType } from "@types";
</script>

<svelte:head>
	<title>Dashboard</title>
  <meta name="description" content="Your personal NRM Dashboard" />
</svelte:head>

<div id="dashboard">
  <!-- <Card type="elevated">
    <div slot="header" class="card-header"></div>
    <div class="body">
      Lorem ipsum dolor sit amet consectetur, adipisicing elit. Ad aliquam accusantium in quas saepe quisquam quia, itaque molestiae quaerat, delectus blanditiis earum, unde facere voluptates maiores? Eaque atque saepe, provident amet nostrum quo dolore. Quibusdam neque possimus reiciendis qui eaque!
    </div>
  </Card> -->

  <PathField name="Test Path" type={FileSelectionType.FOLDER} value="C:/Users" />
</div>

<style>
  #dashboard {

  }
</style>