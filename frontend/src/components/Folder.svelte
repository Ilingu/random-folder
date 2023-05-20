<script lang="ts">
  export let [winnerFolder, firstImage] = ["", ""];
  export let PickRandom: () => Promise<void>;
  export let OpenSubDir: () => Promise<void>;
  export let Reset: () => void;

  let name: string;
  $: name = winnerFolder.split("/").at(-1);
</script>

<div class="card bg-base-300 shadow-xl p-5">
  {#if firstImage != ""}
    <img
      class="border-4 border-primary-focus max-w-md rounded-lg shadow-2xl select-none"
      src={`${winnerFolder}/${firstImage}`}
      alt="subdirectory illustration"
    />
  {/if}
  <div class="card-body">
    <h2
      class="card-title max-w-sm overflow-ellipsis overflow-hidden whitespace-nowrap"
      title={name}
    >
      {name}
    </h2>
    <div class="card-actions justify-evenly">
      <button class="btn btn-primary" on:click={OpenSubDir}>Open 👀</button>
      <button class="btn btn-primary" on:click={PickRandom}>Next 🔀</button>
      <button class="btn btn-primary" on:click={Reset}>Reset 🔄</button>
    </div>
  </div>
</div>
