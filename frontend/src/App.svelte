<script lang="ts">
  import {
    SetFolder,
    GetRandomSubdirPath,
    ResetFolder,
    OpenWinner,
  } from "../wailsjs/go/main/App.js";
  import { decodeReponse } from "./utils";
  import InitPage from "./components/InitPage.svelte";
  import Folder from "./components/Folder.svelte";

  let error: string = null;
  const setError = (msg: string) => {
    error = msg;
    setTimeout(() => (error = null), 5000);
  };

  let initialized = false;
  const initSession = async () => {
    let { succeed, reason } = decodeReponse<string>(await SetFolder());
    initialized = succeed;
    if (succeed) PickRandom();
    else reason && setError(reason);
  };

  let winnerFolderRes: string[] = [];
  const PickRandom = async () => {
    let { succeed, data, reason } = decodeReponse<string[]>(
      await GetRandomSubdirPath()
    );
    if (succeed) winnerFolderRes = data;
    else reason && setError(reason);
  };

  const OpenSubDir = async () => {
    let { succeed, reason } = decodeReponse(await OpenWinner());
    if (!succeed && reason) setError(reason);
  };

  const Reset = () => {
    ResetFolder();
    initialized = false;
  };
</script>

<main class="w-full h-full flex justify-center items-center">
  {#if initialized}
    <Folder
      winnerFolder={winnerFolderRes[0]}
      firstImage={winnerFolderRes[1]}
      {PickRandom}
      {OpenSubDir}
      {Reset}
    />
  {:else}
    <InitPage {initSession} />
  {/if}

  {#if error != null && error.length > 0}
    <div class="toast toast-top toast-end">
      <div class="alert alert-error">
        <div>
          <span>{error}</span>
        </div>
      </div>
    </div>
  {/if}
</main>
