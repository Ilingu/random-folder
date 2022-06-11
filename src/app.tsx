import { useEffect, useRef, useCallback, useState } from "preact/hooks";
import { OpenDialog, ReadDir, ReadImage } from "./utils/fsUtils";
import Form from "./components/Form";
import ImageRenderer from "./components/ImageRenderer";
import { EmptyString, GetLSItem, SetLSItem } from "./utils/utils";
import DisplayWinner from "./components/DisplayWinner";

const App = () => {
  /* State */
  const [FolderPath, setFolderPath] = useState<string>();
  const [ImgUrl, setImgUrl] = useState<string>();
  const [SubFolders, setSubFolders] = useState<string[]>();

  const [WinnerFolder, setWinnerFolder] = useState<string>();

  const SkipQuery = useRef(false);
  const PickWinnerOnUpdate = useRef(false);
  /* Life Cycle */
  useEffect(() => {
    const { success: GetFolderSuccess, returns: FolderPath } =
      GetLSItem("CachedFolderPath");
    if (!GetFolderSuccess || !FolderPath) return;
    if (EmptyString(FolderPath)) return;

    const { success: GetSubSuccess, returns: SubFolders } =
      GetLSItem("CachedSubFolders");

    if (GetSubSuccess) {
      SkipQuery.current = true;
      PickWinnerOnUpdate.current = true;
      setSubFolders(GetSubSuccess && SubFolders);
    }
    setFolderPath(FolderPath);
  }, []);

  useEffect(() => {
    FolderPath && QueryFolder();
  }, [FolderPath]);

  useEffect(() => {
    if (!SubFolders || SubFolders.length <= 0 || !PickWinnerOnUpdate.current)
      return;
    PickWinnerOnUpdate.current = false;
    PickWinner();
  }, [SubFolders]);

  /* FUNCTIONS */
  const PickWinner = (): void => {
    if (!SubFolders) return;
    const WinnerIndex = Math.round(Math.random() * (SubFolders.length - 1));
    const WinnerFolder = SubFolders[WinnerIndex];

    if (SubFolders.length <= 0) {
      console.log("Session Finished");
      return QueryFolder() as unknown as void;
    }

    setWinnerFolder(WinnerFolder);
    QueryImage(WinnerFolder);

    SubFolders.splice(WinnerIndex, 1);
    setSubFolders(SubFolders);
  };

  const QueryFolder = async () => {
    if (!FolderPath) return;

    const { success, returns: SubFoldersData } = await ReadDir(FolderPath);
    if (!success || !SubFoldersData) return;

    const SubFoldersRes = SubFoldersData.map(({ path }) => {
      if (
        path.includes(".jpg") ||
        path.includes(".png") ||
        path.includes(".gif") ||
        path.includes(".mp4") ||
        path.includes(".ini")
      )
        return;
      return path;
    }).filter((d) => d);

    SetLSItem("CachedSubFolders", SubFoldersRes);
    if (SkipQuery.current)
      return (SkipQuery.current = false) as unknown as void;
    PickWinnerOnUpdate.current = true;
    setSubFolders(SubFoldersRes as string[]);
  };

  const ChooseFolder = useCallback(async () => {
    const { success, returns: FolderPath } = await OpenDialog();
    if (!success || !FolderPath) return;

    setFolderPath(FolderPath);
    SetLSItem("CachedFolderPath", FolderPath);
  }, []);

  const QueryImage = async (path: string) => {
    setImgUrl(undefined);
    const { success, returns: Image } = await ReadImage(path);
    if (!success) return;
    setImgUrl(Image);
  };

  /* JSX */
  return (
    <main>
      <header>
        <h1 id="Title">Random File Picker 🔀</h1>
      </header>
      {WinnerFolder ? (
        <DisplayWinner
          FolderPath={WinnerFolder}
          ChooseFolder={ChooseFolder}
          PickWinner={PickWinner}
        />
      ) : (
        <Form ChooseFolder={ChooseFolder} />
      )}
      {/* IMG */}
      <ImageRenderer ImageUrl={ImgUrl} />
    </main>
  );
};

export default App;
