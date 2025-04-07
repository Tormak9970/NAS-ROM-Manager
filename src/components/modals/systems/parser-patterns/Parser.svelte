<script lang="ts">
  import { TextField } from "@interactables";
  import { Accordion } from "@layout";
  import type { ParserPattern } from "@types";

  type Props = {
    label: string;
    pattern: ParserPattern;
  }

  let {
    label,
    pattern = $bindable()
  }: Props = $props();
</script>

<Accordion label={label}>
  <div class="pattern">
    <TextField
      name="Glob"
      bind:value={pattern.glob}
    />
    <div class="footnote">
      An outline of the glob syntax can be found <a href="https://github.com/olson-sean-k/wax?tab=readme-ov-file#patterns" target="_blank" rel="noreferrer noopenner">here</a>.
    </div>
    <TextField
      name="Regex"
      bind:value={pattern.regex}
    />
    <div class="footnote">
      To test and learn about RegEx, check out <a href="https://regex101.com/" target="_blank" rel="noreferrer noopenner">https://regex101.com/</a>.
    </div>
    <!-- TODO: download strategy -->
    
    <div class="examples">
      For examples, take a look at the <a href="https://github.com/Tormak9970/NAS-ROM-Manager/tree/main/parsers" target="_blank" rel="noreferrer noopenner">default parsers</a>.
    </div>
  </div>
</Accordion>

<style>
  .footnote {
    margin-bottom: 0.25rem;
  }

  .examples {
    margin-top: 1rem;
  }
</style>