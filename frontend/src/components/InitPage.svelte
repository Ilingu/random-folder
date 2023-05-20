<script lang="ts">
  import logo from "../assets/images/logo-universal.png";
  export let initSession: () => Promise<void>;
</script>

<div class="hero min-h-screen bg-base-200">
  <div class="hero-content flex-col lg:flex-row-reverse">
    <img alt="App logo" class="max-w-sm rounded-lg shadow-2xl" src={logo} />
    <div>
      <h1 class="text-5xl text-primary font-bold">Random folder picker</h1>
      <p class="py-6">
        Random Folder is an <s>lightweight and blazingly fast</s> app that pick a
        random subfolder inside of a main folder. To begin using this app; choose
        a folder containing sub-folders to pick from
      </p>
      <button class="btn btn-primary w-1/2" on:click={initSession}
        >📂 Set folder</button
      >
    </div>
  </div>
</div>
