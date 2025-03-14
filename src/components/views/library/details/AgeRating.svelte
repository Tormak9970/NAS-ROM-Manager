<script lang="ts">
  import { RATINGS_COUNTRIES, type IGDBAgeRating } from "@types";

  type Props = {
    rating: IGDBAgeRating;
  }

  let { rating }: Props = $props();
</script>

<div class="age-rating">
  <div class="country">{RATINGS_COUNTRIES[rating.category]}</div>
  <img src={rating.ratingCoverUrl} alt={rating.rating} />
</div>

<style>
  .age-rating {
    background-color: rgb(var(--m3-scheme-surface-container-high));

    padding: 0.25rem;

    border-radius: var(--m3-util-rounding-small);

    display: flex;
    flex-direction: column;
    align-items: center;

    width: 5rem;

    gap: 0.25rem;
  }

  .country {
    font-weight: bold;
  }

  img {
    max-height: 64px;
  }
</style>